mod deps;
mod gpu;
mod input;
mod surface;

use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::winit::RawWinitWindowEvent;
use crossbeam_channel::{Receiver, unbounded};
use fxhash::FxHashMap;
use graphix_compiler::{env::Env, expr::ModuleResolver, BindId};
use graphix_package_gui::{
    widgets::Message,
    window::ResolvedWindow,
};
use graphix_rt::{GXConfig, GXEvent, GXHandle, GXRt, NoExt};
use iced_core::clipboard;
use log::error;
use netidx::{protocol::valarray::ValArray, publisher::Value};
use poolshark::{global::GPooled, local::LPooled};
use std::cell::RefCell;

use crate::net::NetInfra;
use gpu::IcedGpu;
use surface::IcedSurface;

/// Handle to the running Graphix runtime.
#[derive(Resource)]
pub struct GraphixRuntime {
    handle: GXHandle<NoExt>,
    events: Receiver<GPooled<Vec<GXEvent>>>,
}

impl GraphixRuntime {
    pub fn handle(&self) -> &GXHandle<NoExt> {
        &self.handle
    }
}

fn start_graphix(infra: &NetInfra) -> Result<GraphixRuntime> {
    let publisher = infra.publisher();
    let subscriber = infra.subscriber();
    let (event_tx, event_rx) = unbounded();
    let handle = infra.handle().block_on(async {
        let mut ctx = graphix_compiler::ExecCtx::new(GXRt::<NoExt>::new(publisher, subscriber))
            .context("creating graphix context")?;
        let mut vfs_modules = FxHashMap::default();
        let result = deps::register::<NoExt>(&mut ctx, &mut vfs_modules)
            .context("register package modules")?;
        let mods = vec![ModuleResolver::VFS(vfs_modules)];
        let (tokio_tx, mut tokio_rx) = tokio::sync::mpsc::channel(100);
        let event_tx_clone = event_tx.clone();
        tokio::spawn(async move {
            while let Some(batch) = tokio_rx.recv().await {
                let _ = event_tx_clone.send(batch);
            }
        });
        let handle = GXConfig::builder(ctx, tokio_tx)
            .root(result.root)
            .resolvers(mods)
            .build()
            .context("building graphix config")?
            .start()
            .await
            .context("starting graphix runtime")?;
        Ok::<_, anyhow::Error>(handle)
    })?;
    Ok(GraphixRuntime { handle, events: event_rx })
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
                    cb.get()
                        .clipboard(arboard::LinuxClipboardKind::Primary)
                        .text()
                        .ok()
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
    input_router: input::InputRouter,
    clipboard: Clipboard,
    messages: Vec<Message>,
    env: Option<Env>,
}

/// Reconcile surfaces with a new root array value.
fn reconcile_surfaces(
    state: &mut GraphixGuiState,
    gx: &GXHandle<NoExt>,
    rt: &tokio::runtime::Handle,
    root_value: Value,
    images: &mut Assets<Image>,
) -> Result<()> {
    let arr = root_value
        .cast_to::<LPooled<Vec<u64>>>()
        .context("root array of bind ids")?;
    let new_bids: Vec<BindId> = arr.iter().map(|&id| BindId::from(id)).collect();

    // Remove surfaces no longer in the array
    let to_remove: Vec<BindId> = state
        .surfaces
        .keys()
        .filter(|bid| !new_bids.contains(bid))
        .copied()
        .collect();
    for bid in &to_remove {
        state.surfaces.remove(bid);
    }

    // Add new surfaces
    for &bid in &new_bids {
        if state.surfaces.contains_key(&bid) {
            continue;
        }
        let wref = rt
            .block_on(gx.compile_ref(bid))
            .context("compile_ref for window bind id")?;
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
        let surface = IcedSurface::new(&state.gpu, resolved, wref, w.max(1), h.max(1), images);
        state.surfaces.insert(bid, surface);
    }

    Ok(())
}

/// Check if a value looks like a gui root (array of u64 bind IDs).
fn is_gui_root_update(v: &Value) -> bool {
    v.clone().cast_to::<LPooled<Vec<u64>>>().is_ok()
}

/// Main update system — drains graphix events, reconciles surfaces, renders.
fn graphix_gui_update(
    runtime: Res<GraphixRuntime>,
    mut gui_state: NonSendMut<GraphixGuiState>,
    mut raw_winit_events: bevy::ecs::message::MessageReader<RawWinitWindowEvent>,
    mut images: ResMut<Assets<Image>>,
) {
    let state = &mut *gui_state;
    let rt_handle = tokio::runtime::Handle::current();

    // Drain graphix events
    let mut root_update: Option<Value> = None;
    for batch in runtime.events.try_iter() {
        for event in batch.iter() {
            match event {
                GXEvent::Updated(id, v) => {
                    if is_gui_root_update(v) {
                        root_update = Some(v.clone());
                    } else {
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

    // Route input events to surfaces
    let raw_events: Vec<RawWinitWindowEvent> = raw_winit_events.read().cloned().collect();
    if !raw_events.is_empty() && !state.surfaces.is_empty() {
        let mut surface_refs: Vec<&mut IcedSurface> = state.surfaces.values_mut().collect();
        state.input_router.route_events(&raw_events, &mut surface_refs);
    }

    // Render dirty surfaces
    for surface in state.surfaces.values_mut() {
        surface.render(
            &mut state.clipboard,
            &mut state.messages,
            &state.gpu,
            &mut images,
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

/// Marker for the GUI overlay UI node.
#[derive(Component)]
struct GuiOverlay;

/// Keeps track of which surfaces already have an overlay node.
#[derive(Resource, Default)]
struct OverlayState {
    /// BindId → Entity for overlay nodes
    overlays: FxHashMap<BindId, Entity>,
}

/// Sync overlay UI nodes with active surfaces.
fn sync_overlays(
    gui_state: NonSend<GraphixGuiState>,
    mut overlay_state: ResMut<OverlayState>,
    mut commands: Commands,
) {
    // Add overlays for new surfaces
    for (&bid, surface) in &gui_state.surfaces {
        if overlay_state.overlays.contains_key(&bid) {
            continue;
        }
        let entity = commands
            .spawn((
                GuiOverlay,
                ImageNode {
                    image: surface.bevy_image.clone(),
                    ..default()
                },
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
            ))
            .id();
        overlay_state.overlays.insert(bid, entity);
    }

    // Remove overlays for removed surfaces
    let to_remove: Vec<BindId> = overlay_state
        .overlays
        .keys()
        .filter(|bid| !gui_state.surfaces.contains_key(bid))
        .copied()
        .collect();
    for bid in to_remove {
        if let Some(entity) = overlay_state.overlays.remove(&bid) {
            commands.entity(entity).despawn();
        }
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
            .init_resource::<OverlayState>()
            .add_systems(Update, (graphix_gui_update, sync_overlays).chain());
    }

    fn finish(&self, app: &mut App) {
        // Extract wgpu resources from Bevy's render sub-app
        let render_app = app
            .get_sub_app(bevy::render::RenderApp)
            .expect("RenderApp not found — is DefaultPlugins added before GraphixPlugin?");
        let render_device = render_app
            .world()
            .resource::<bevy::render::renderer::RenderDevice>();
        let render_queue = render_app
            .world()
            .resource::<bevy::render::renderer::RenderQueue>();
        let render_adapter = render_app
            .world()
            .resource::<bevy::render::renderer::RenderAdapter>();

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
            input_router: input::InputRouter::new(),
            clipboard: Clipboard::new(),
            messages: Vec::new(),
            env: None,
        };

        app.insert_non_send_resource(gui_state);
    }
}
