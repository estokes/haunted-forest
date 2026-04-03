mod deps;
mod gpu;
mod surface;

use anyhow::{Context, Result};
use arcstr::ArcStr;
use bevy::ecs::message::MessageReader;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::winit::RawWinitWindowEvent;
use crossbeam_channel::{unbounded, Receiver};
use fxhash::FxHashMap;
use graphix_compiler::{env::Env, expr::{ExprId, ModuleResolver}, BindId};
use graphix_package_gui::{convert, widgets::Message, window::ResolvedWindow};
use graphix_rt::{GXConfig, GXEvent, GXHandle, GXRt, NoExt};
use iced_core::clipboard;
use log::error;
use netidx::{protocol::valarray::ValArray, publisher::Value};
use poolshark::{global::GPooled, local::LPooled};
use std::cell::RefCell;
use winit::keyboard::ModifiersState;

use crate::GameState;
use crate::net::NetInfra;
use crate::player::PlayerCamera;
use crate::ship::GuiScreen;
use gpu::IcedGpu;
use surface::IcedSurface;

/// Handle to the running Graphix runtime.
#[derive(Resource)]
pub struct GraphixRuntime {
    handle: GXHandle<NoExt>,
    events: Receiver<GPooled<Vec<GXEvent>>>,
    rt_handle: tokio::runtime::Handle,
}

impl GraphixRuntime {
    pub fn handle(&self) -> &GXHandle<NoExt> {
        &self.handle
    }

    pub fn rt(&self) -> &tokio::runtime::Handle {
        &self.rt_handle
    }
}

#[derive(Resource, Default)]
pub struct ScreenInteraction {
    pub screen_entity: Option<Entity>,
    pub cursor_pos: Vec2,
    pub modifiers: ModifiersState,
}

const HELM_GX: &str = include_str!("../../../../assets/gui/helm.gx");

fn start_graphix(infra: &NetInfra) -> Result<GraphixRuntime> {
    let publisher = infra.publisher();
    let subscriber = infra.subscriber();
    let (event_tx, event_rx) = unbounded();
    let handle = infra.handle().block_on(async {
        let mut ctx =
            graphix_compiler::ExecCtx::new(GXRt::<NoExt>::new(publisher, subscriber))
                .context("creating graphix context")?;
        let mut vfs_modules = FxHashMap::default();
        let result = deps::register::<NoExt>(&mut ctx, &mut vfs_modules)
            .context("register package modules")?;
        let mods = vec![ModuleResolver::VFS(vfs_modules)];
        let (tokio_tx, mut tokio_rx) = tokio::sync::mpsc::channel(100);
        let event_tx_clone = event_tx.clone();
        tokio::spawn(async move {
            eprintln!("DBG bridge task started");
            while let Some(batch) = tokio_rx.recv().await {
                eprintln!("DBG bridge task forwarding batch");
                let _ = event_tx_clone.send(batch);
            }
            eprintln!("DBG bridge task exited");
        });
        let root = ArcStr::from(format!("{};\n{}", result.root, HELM_GX));
        eprintln!("DBG root len={}", root.len());
        let handle = GXConfig::builder(ctx, tokio_tx)
            .root(root)
            .resolvers(mods)
            .build()
            .context("building graphix config")?
            .start()
            .await
            .context("starting graphix runtime")?;
        Ok::<_, anyhow::Error>(handle)
    })?;
    Ok(GraphixRuntime {
        handle,
        events: event_rx,
        rt_handle: infra.handle().clone(),
    })
}

/// System clipboard backed by arboard.
struct Clipboard {
    state: RefCell<Option<arboard::Clipboard>>,
}

impl Clipboard {
    fn new() -> Self {
        Self { state: RefCell::new(arboard::Clipboard::new().ok()) }
    }
}

impl clipboard::Clipboard for Clipboard {
    fn read(&self, kind: clipboard::Kind) -> Option<String> {
        let mut cb = self.state.borrow_mut();
        let cb = cb.as_mut()?;
        match kind {
            clipboard::Kind::Standard => cb.get_text().ok(),
            clipboard::Kind::Primary => {
                #[cfg(target_os = "linux")]
                {
                    use arboard::GetExtLinux;
                    cb.get().clipboard(arboard::LinuxClipboardKind::Primary).text().ok()
                }
                #[cfg(not(target_os = "linux"))]
                None
            }
        }
    }

    fn write(&mut self, kind: clipboard::Kind, contents: String) {
        let mut cb = self.state.borrow_mut();
        let Some(cb) = cb.as_mut() else { return };
        match kind {
            clipboard::Kind::Standard => {
                let _ = cb.set_text(contents);
            }
            clipboard::Kind::Primary => {
                #[cfg(target_os = "linux")]
                {
                    use arboard::SetExtLinux;
                    let _ = cb
                        .set()
                        .clipboard(arboard::LinuxClipboardKind::Primary)
                        .text(contents);
                }
            }
        }
    }
}

/// All GUI state. NonSend because iced_wgpu::Renderer may be !Send.
pub struct GraphixGuiState {
    gpu: IcedGpu,
    surfaces: FxHashMap<BindId, IcedSurface>,
    clipboard: Clipboard,
    messages: Vec<Message>,
    env: Option<Env>,
    root_expr_id: Option<ExprId>,
}

/// Reconcile surfaces with a new root array value.
fn reconcile_surfaces(
    state: &mut GraphixGuiState,
    gx: &GXHandle<NoExt>,
    rt: &tokio::runtime::Handle,
    root_value: Value,
    images: &mut Assets<Image>,
) -> Result<()> {
    // CR estokes: You can make the gui type public in the gui package
    // and then do the proper type compare here.
    //
    // We eventually need to be able to give the user a Graphix
    // shell. We can't use the actual shell because we're not in a
    // terminal, but the point is the rval for a gui should match the
    // type.
    let arr =
        root_value.cast_to::<LPooled<Vec<u64>>>().context("root array of bind ids")?;
    let new_bids: Vec<BindId> = arr.iter().map(|&id| BindId::from(id)).collect();

    // Remove surfaces no longer in the array
    let to_remove: Vec<BindId> =
        state.surfaces.keys().filter(|bid| !new_bids.contains(bid)).copied().collect();
    for bid in &to_remove {
        state.surfaces.remove(bid);
    }

    // Add new surfaces
    for &bid in &new_bids {
        if state.surfaces.contains_key(&bid) {
            continue;
        }
        let wref =
            rt.block_on(gx.compile_ref(bid)).context("compile_ref for window bind id")?;
        let window_value = match wref.last.as_ref() {
            Some(v) => v.clone(),
            None => {
                error!("window bind id {bid:?} has no initial value, skipping");
                continue;
            }
        };
        let resolved = rt
            .block_on(ResolvedWindow::compile(gx.clone(), window_value))
            .context("resolve window")?;
        let (w, h) = resolved
            .size
            .t
            .as_ref()
            .map(|sz| (sz.0.width as u32, sz.0.height as u32))
            .unwrap_or((800, 600));
        let surface =
            IcedSurface::new(&state.gpu, resolved, wref, w.max(1), h.max(1), images);
        state.surfaces.insert(bid, surface);
    }

    Ok(())
}

/// Check if a value looks like a gui root (array of u64 bind IDs).
fn looks_like_gui_root(v: &Value) -> bool {
    matches!(v, Value::Array(_)) && v.clone().cast_to::<LPooled<Vec<u64>>>().is_ok()
}

// Half-extents of the Plane3d mesh (from bridge.rs: Vec2::new(0.9, 0.35))
const HALF_X: f32 = 0.9;
const HALF_Z: f32 = 0.35;

/// Raycast against a screen plane, returning UV (0..1) and distance if hit.
fn raycast_screen(ray: Ray3d, screen_gt: &GlobalTransform) -> Option<(Vec2, f32)> {
    let screen_normal = screen_gt.compute_transform().rotation * Vec3::Y;
    let screen_point = screen_gt.translation();
    let denom = ray.direction.dot(screen_normal);
    if denom.abs() < 1e-6 {
        return None;
    }
    let t = (screen_point - ray.origin).dot(screen_normal) / denom;
    if t < 0.0 {
        return None;
    }
    let hit_world = ray.origin + *ray.direction * t;
    let local = screen_gt.affine().inverse().transform_point3(hit_world);
    if local.x.abs() > HALF_X || local.z.abs() > HALF_Z {
        return None;
    }
    // UV: U along +X, V along -Z (top-left origin for iced).
    let u = (local.x + HALF_X) / (2.0 * HALF_X);
    let v = (HALF_Z - local.z) / (2.0 * HALF_Z);
    Some((Vec2::new(u, v), t))
}

/// Toggle screen interaction: E to focus, Escape to exit.
fn interaction_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    screen_q: Query<(Entity, &GlobalTransform), With<GuiScreen>>,
    mut interaction: ResMut<ScreenInteraction>,
    mut gui_state: NonSendMut<GraphixGuiState>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match state.get() {
        GameState::Playing => {
            if !keys.just_pressed(KeyCode::KeyE) {
                return;
            }
            let Ok((camera, cam_gt)) = camera_q.single() else { return };
            // Ray from camera center (crosshair)
            let Ok(ray) = camera.viewport_to_world(cam_gt, Vec2::new(
                // Use viewport center
                camera.logical_viewport_size().map(|s| s.x / 2.0).unwrap_or(400.0),
                camera.logical_viewport_size().map(|s| s.y / 2.0).unwrap_or(300.0),
            )) else { return };

            let mut best: Option<(Entity, f32)> = None;
            for (entity, screen_gt) in &screen_q {
                if let Some((_, dist)) = raycast_screen(ray, screen_gt)
                    && dist < 3.0
                    && best.is_none_or(|(_, d)| dist < d)
                {
                    best = Some((entity, dist));
                }
            }
            if let Some((entity, _)) = best {
                interaction.screen_entity = Some(entity);
                // Center cursor on the actual surface dimensions
                if let Some(surface) = gui_state.surfaces.values_mut().next() {
                    let cx = surface.width as f32 / 2.0;
                    let cy = surface.height as f32 / 2.0;
                    interaction.cursor_pos = Vec2::new(cx, cy);
                    surface.needs_redraw = true;
                }
                next_state.set(GameState::InteractingWithScreen);
            }
        }
        GameState::InteractingWithScreen => {
            if keys.just_pressed(KeyCode::Escape) {
                interaction.screen_entity = None;
                // Flag surface for redraw so cursor disappears
                for surface in gui_state.surfaces.values_mut() {
                    surface.needs_redraw = true;
                }
                next_state.set(GameState::Playing);
            }
        }
    }
}

/// Route mouse/keyboard input to the focused screen's iced surface.
fn screen_interaction_input(
    mut interaction: ResMut<ScreenInteraction>,
    mut gui_state: NonSendMut<GraphixGuiState>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut raw_events: MessageReader<RawWinitWindowEvent>,
) {
    let state = &mut *gui_state;
    let Some(surface) = state.surfaces.values_mut().next() else { return };

    // Keyboard: convert raw winit events, filter to keyboard only
    for raw in raw_events.read() {
        if let winit::event::WindowEvent::ModifiersChanged(m) = &raw.event {
            interaction.modifiers = m.state();
        }
        // Skip escape — handled by interaction_toggle
        if matches!(
            &raw.event,
            winit::event::WindowEvent::KeyboardInput { event, .. }
                if event.logical_key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape)
        ) {
            continue;
        }
        let iced_events = convert::window_event(&raw.event, 1.0, interaction.modifiers);
        for ev in iced_events.iter() {
            if matches!(ev, iced_core::Event::Keyboard(_)) {
                surface.push_event(ev.clone());
            }
        }
    }

    // Mouse movement: accumulate delta into virtual cursor.
    // cursor_pos is in buffer-space (used for drawing the crosshair).
    // Delta is negated because the readback double-flip mirrors buffer vs screen.
    // Iced needs the inverse-mapped coordinate since iced-space is flipped
    // relative to buffer-space: iced_pos = (width - buffer_x, height - buffer_y).
    let w = surface.width as f32;
    let h = surface.height as f32;
    let delta = mouse_motion.delta;
    if delta != Vec2::ZERO {
        const MARGIN: f32 = 4.0;
        interaction.cursor_pos.x =
            (interaction.cursor_pos.x - delta.x).clamp(MARGIN, w - MARGIN);
        interaction.cursor_pos.y =
            (interaction.cursor_pos.y - delta.y).clamp(MARGIN, h - MARGIN);
        let iced_pos =
            iced_core::Point::new(w - interaction.cursor_pos.x, h - interaction.cursor_pos.y);
        surface.cursor_position = iced_pos;
        surface.push_event(iced_core::Event::Mouse(
            iced_core::mouse::Event::CursorMoved { position: iced_pos },
        ));
    }

    // Mouse buttons
    if mouse_buttons.just_pressed(MouseButton::Left) {
        surface.push_event(iced_core::Event::Mouse(
            iced_core::mouse::Event::ButtonPressed(iced_core::mouse::Button::Left),
        ));
    }
    if mouse_buttons.just_released(MouseButton::Left) {
        surface.push_event(iced_core::Event::Mouse(
            iced_core::mouse::Event::ButtonReleased(iced_core::mouse::Button::Left),
        ));
    }
    if mouse_buttons.just_pressed(MouseButton::Right) {
        surface.push_event(iced_core::Event::Mouse(
            iced_core::mouse::Event::ButtonPressed(iced_core::mouse::Button::Right),
        ));
    }
    if mouse_buttons.just_released(MouseButton::Right) {
        surface.push_event(iced_core::Event::Mouse(
            iced_core::mouse::Event::ButtonReleased(iced_core::mouse::Button::Right),
        ));
    }
}

/// Main update system — drains graphix events, reconciles surfaces, renders.
fn graphix_gui_update(
    runtime: Res<GraphixRuntime>,
    mut gui_state: NonSendMut<GraphixGuiState>,
    mut images: ResMut<Assets<Image>>,
    interaction: Res<ScreenInteraction>,
    game_state: Res<State<GameState>>,
) {
    let state = &mut *gui_state;
    let rt_handle = runtime.rt().clone();
    let _guard = rt_handle.enter();

    // One-shot debug: print on first few frames
    static DBG_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let frame = DBG_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if frame < 300 && frame % 60 == 0 {
        let pending = runtime.events.len();
        eprintln!("DBG frame={frame} surfaces={} pending_events={pending}", state.surfaces.len());
    }

    // Drain graphix events
    let mut root_update: Option<Value> = None;
    let mut event_count = 0_usize;
    let mut root_count = 0_usize;
    let mut non_root_count = 0_usize;
    for batch in runtime.events.try_iter() {
        for event in batch.iter() {
            event_count += 1;
            match event {
                GXEvent::Updated(id, v) => {
                    let is_root = match state.root_expr_id {
                        Some(rid) => *id == rid,
                        None if looks_like_gui_root(v) => {
                            eprintln!("DBG captured root_expr_id={id:?}");
                            state.root_expr_id = Some(*id);
                            true
                        }
                        None => false,
                    };
                    if is_root {
                        root_count += 1;
                        root_update = Some(v.clone());
                    } else {
                        non_root_count += 1;
                        for surface in state.surfaces.values_mut() {
                            if let Err(e) = surface.handle_update(&rt_handle, *id, v) {
                                error!("surface handle_update: {e:?}");
                            }
                        }
                    }
                }
                GXEvent::Env(env) => {
                    state.env = Some(env.clone());
                }
            }
        }
    }

    if event_count > 0 {
        eprintln!("DBG drained {event_count} events (root={root_count} content={non_root_count}), surfaces={}", state.surfaces.len());
    }

    // Reconcile surfaces if root updated
    if let Some(root_value) = root_update
        && let Err(e) = reconcile_surfaces(
            state,
            runtime.handle(),
            &rt_handle,
            root_value,
            &mut images,
        )
    {
        error!("reconcile surfaces: {e:?}");
    }

    // Render dirty surfaces
    let cursor_overlay = if *game_state.get() == GameState::InteractingWithScreen {
        Some(interaction.cursor_pos)
    } else {
        None
    };
    for (bid, surface) in state.surfaces.iter_mut() {
        if surface.needs_redraw {
            eprintln!(
                "DBG rendering surface {bid:?}: {}x{}, events={}",
                surface.width, surface.height, surface.pending_events.len()
            );
        }
        surface.render(
            &mut state.clipboard,
            &mut state.messages,
            &state.gpu,
            &mut images,
            cursor_overlay,
        );
    }

    // Process iced messages
    for msg in state.messages.drain(..) {
        match msg {
            Message::Nop => {}
            Message::Call(id, args) => {
                if let Err(e) = runtime.handle().call(id, args) {
                    error!("failed to call: {e:?}");
                }
            }
            Message::EditorAction(id, action) => {
                for surface in state.surfaces.values_mut() {
                    if let Some((callable_id, v)) = surface.editor_action(id, &action) {
                        if let Err(e) =
                            runtime.handle().call(callable_id, ValArray::from_iter([v]))
                        {
                            error!("failed to call editor callback: {e:?}");
                        }
                        break;
                    }
                }
            }
        }
    }
}

/// Update GuiScreen materials to display graphix surface textures.
fn sync_screens(
    mut gui_state: NonSendMut<GraphixGuiState>,
    screen_q: Query<(Entity, &MeshMaterial3d<StandardMaterial>), With<GuiScreen>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some(surface) = gui_state.surfaces.values_mut().next() else { return };
    if !surface.new_image {
        return;
    }
    surface.new_image = false;
    let handle = surface.bevy_image.clone();
    for (_entity, mat_handle) in &screen_q {
        let Some(mat) = materials.get_mut(mat_handle) else { continue };
        mat.base_color_texture = Some(handle.clone());
        mat.base_color = Color::WHITE;
        mat.emissive = LinearRgba::NONE;
    }
}

pub struct GraphixPlugin;

impl Plugin for GraphixPlugin {
    fn build(&self, app: &mut App) {
        let infra = app
            .world()
            .get_resource::<NetInfra>()
            .expect("GraphixPlugin requires NetPlugin to be added first");
        let runtime = start_graphix(infra).expect("failed to start graphix runtime");
        app.insert_resource(runtime)
            .init_resource::<ScreenInteraction>()
            .add_systems(
                Update,
                (
                    interaction_toggle,
                    screen_interaction_input
                        .run_if(in_state(GameState::InteractingWithScreen)),
                    graphix_gui_update,
                    sync_screens,
                )
                    .chain(),
            );
    }

    fn finish(&self, app: &mut App) {
        // Extract wgpu resources from Bevy's render sub-app
        let render_app = app.get_sub_app(bevy::render::RenderApp).expect(
            "RenderApp not found — is DefaultPlugins added before GraphixPlugin?",
        );
        let render_device =
            render_app.world().resource::<bevy::render::renderer::RenderDevice>();
        let render_queue =
            render_app.world().resource::<bevy::render::renderer::RenderQueue>();
        let render_adapter =
            render_app.world().resource::<bevy::render::renderer::RenderAdapter>();

        // RenderQueue is Arc<WgpuWrapper<Queue>>, deref through Arc then WgpuWrapper
        let raw_queue: &iced_wgpu::wgpu::Queue = &render_queue.0;
        let raw_adapter: &iced_wgpu::wgpu::Adapter = &render_adapter.0;
        let iced_gpu = IcedGpu {
            device: render_device.wgpu_device().clone(),
            queue: raw_queue.clone(),
            adapter: raw_adapter.clone(),
            format: iced_wgpu::wgpu::TextureFormat::Bgra8UnormSrgb,
        };

        let gui_state = GraphixGuiState {
            gpu: iced_gpu,
            surfaces: FxHashMap::default(),
            clipboard: Clipboard::new(),
            messages: Vec::new(),
            root_expr_id: None,
            env: None,
        };

        app.insert_non_send_resource(gui_state);
    }
}
