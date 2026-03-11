use anyhow::{Context, Result};
use bevy::prelude::*;
use graphix_compiler::expr::ExprId;
use graphix_package_gui::{
    render::GpuState,
    theme::GraphixTheme,
    types::{SizeV, ThemeV},
    widgets::{compile, GuiW, Message},
    window::ResolvedWindow,
};
use graphix_rt::{GXHandle, NoExt, Ref, TRef};
use iced_core::{clipboard, mouse, renderer::Style, Size};
use iced_runtime::user_interface::{self, UserInterface};
use iced_wgpu::{graphics::Viewport, wgpu};
use netidx::publisher::Value;

use super::gpu::IcedGpu;

/// One per GUI panel — holds an offscreen iced rendering surface.
pub struct IcedSurface {
    pub renderer: iced_wgpu::Renderer,
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    staging_buffer: wgpu::Buffer,
    viewport: Viewport,
    pub(crate) width: u32,
    pub(crate) height: u32,
    cache: user_interface::Cache,
    // Graphix refs
    gx: GXHandle<NoExt>,
    window_ref: Ref<NoExt>,
    title: TRef<NoExt, String>,
    size: TRef<NoExt, SizeV>,
    theme: TRef<NoExt, ThemeV>,
    content_ref: Ref<NoExt>,
    content: GuiW<NoExt>,
    // Input state
    pub pending_events: Vec<iced_core::Event>,
    pub cursor_position: iced_core::Point,
    pub needs_redraw: bool,
    // Bevy image handle
    pub bevy_image: Handle<Image>,
    pub new_image: bool,
}

/// Bytes per row, aligned to wgpu's COPY_BYTES_PER_ROW_ALIGNMENT.
fn aligned_bytes_per_row(width: u32) -> u32 {
    let unaligned = width * 4;
    unaligned.div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT) * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT
}

fn create_offscreen_texture(
    device: &wgpu::Device,
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("iced_offscreen"),
        size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view)
}

fn create_staging_buffer(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Buffer {
    let size = (aligned_bytes_per_row(width) * height) as u64;
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("iced_staging"),
        size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    })
}

impl IcedSurface {
    pub fn new(
        gpu: &IcedGpu,
        resolved: ResolvedWindow<NoExt>,
        window_ref: Ref<NoExt>,
        width: u32,
        height: u32,
        images: &mut Assets<Image>,
    ) -> Self {
        let (texture, texture_view) = create_offscreen_texture(
            &gpu.device,
            width,
            height,
            gpu.format,
        );
        let staging_buffer = create_staging_buffer(&gpu.device, width, height);
        let viewport = Viewport::with_physical_size(
            Size::new(width, height),
            1.0, // scale factor 1.0 for offscreen
        );
        let renderer = {
            let gpu_state = GpuState {
                instance: wgpu::Instance::default(),
                adapter: gpu.adapter.clone(),
                device: gpu.device.clone(),
                queue: gpu.queue.clone(),
                format: gpu.format,
            };
            gpu_state.create_renderer()
        };
        // Create a blank Bevy Image for this surface
        let bevy_img = Image::new_fill(
            bevy::render::render_resource::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            bevy::render::render_resource::TextureDimension::D2,
            &[0, 0, 0, 255],
            bevy::render::render_resource::TextureFormat::Bgra8UnormSrgb,
            bevy::asset::RenderAssetUsages::MAIN_WORLD | bevy::asset::RenderAssetUsages::RENDER_WORLD,
        );
        let bevy_image = images.add(bevy_img);
        IcedSurface {
            renderer,
            texture,
            texture_view,
            staging_buffer,
            viewport,
            width,
            height,
            cache: user_interface::Cache::default(),
            gx: resolved.gx,
            window_ref,
            title: resolved.title,
            size: resolved.size,
            theme: resolved.theme,
            content_ref: resolved.content_ref,
            content: resolved.content,
            pending_events: Vec::new(),
            cursor_position: iced_core::Point::ORIGIN,
            needs_redraw: true,
            bevy_image,
            new_image: true,
        }
    }

    fn iced_theme(&self) -> GraphixTheme {
        self.theme
            .t
            .as_ref()
            .map(|t| t.0.clone())
            .unwrap_or(GraphixTheme {
                inner: iced_core::Theme::Dark,
                overrides: None,
            })
    }

    fn cursor(&self) -> mouse::Cursor {
        mouse::Cursor::Available(self.cursor_position)
    }

    /// Run the iced render cycle: build UI → update → draw → present → readback.
    pub fn render(
        &mut self,
        clipboard: &mut dyn clipboard::Clipboard,
        messages: &mut Vec<Message>,
        gpu: &IcedGpu,
        images: &mut Assets<Image>,
    ) {
        if !self.needs_redraw {
            return;
        }
        let cursor = self.cursor();
        let theme = self.iced_theme();
        let cache = std::mem::take(&mut self.cache);
        let element = self.content.view();
        let viewport_size = self.viewport.logical_size();
        let mut ui = UserInterface::build(element, viewport_size, cache, &mut self.renderer);
        let (state, _statuses) = ui.update(
            &self.pending_events,
            cursor,
            &mut self.renderer,
            clipboard,
            messages,
        );
        let style = Style { text_color: theme.palette().text };
        ui.draw(&mut self.renderer, &theme, &style, cursor);
        self.cache = ui.into_cache();
        self.pending_events.clear();
        // Present to offscreen texture (clear with theme background to avoid overdraw)
        self.renderer.present(
            Some(theme.palette().background),
            gpu.format,
            &self.texture_view,
            &self.viewport,
        );
        // Readback pixels to update the Bevy Image
        self.readback_to_image(gpu, images);
        self.needs_redraw = matches!(
            state,
            user_interface::State::Outdated
                | user_interface::State::Updated {
                    redraw_request: iced_core::window::RedrawRequest::NextFrame,
                    ..
                }
        );
    }

    /// Copy the offscreen texture to a staging buffer and read pixels into the Bevy Image.
    fn readback_to_image(&mut self, gpu: &IcedGpu, images: &mut Assets<Image>) {
        let bpr = aligned_bytes_per_row(self.width);
        let mut encoder =
            gpu.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("iced_readback"),
                });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &self.staging_buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(bpr),
                    rows_per_image: None,
                },
            },
            wgpu::Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
        );
        gpu.queue.submit(Some(encoder.finish()));
        // Map and read the staging buffer synchronously
        let slice = self.staging_buffer.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        let _ = gpu.device.poll(wgpu::PollType::wait_indefinitely());
        {
            let mapped = slice.get_mapped_range();
            let row_bytes = (self.width * 4) as usize;
            let mut pixels = Vec::with_capacity(row_bytes * self.height as usize);
            for row in (0..self.height as usize).rev() {
                let start = row * bpr as usize;
                let row_data = &mapped[start..start + row_bytes];
                // Reverse pixel order within the row (4 bytes per pixel)
                for px in (0..row_bytes).rev().step_by(4) {
                    pixels.extend_from_slice(&row_data[px - 3..=px]);
                }
            }
            drop(mapped);
            self.staging_buffer.unmap();
            let mut new_image = Image::new_fill(
                bevy::render::render_resource::Extent3d {
                    width: self.width,
                    height: self.height,
                    depth_or_array_layers: 1,
                },
                bevy::render::render_resource::TextureDimension::D2,
                &[0, 0, 0, 255],
                bevy::render::render_resource::TextureFormat::Bgra8UnormSrgb,
                bevy::asset::RenderAssetUsages::RENDER_WORLD,
            );
            new_image.data = Some(pixels);
            // Replace the old handle with the new image and update bevy_image
            self.bevy_image = images.add(new_image);
            self.new_image = true;
        }
    }

    /// Handle a graphix value update. Returns true if the surface needs redraw.
    pub fn handle_update(
        &mut self,
        rt: &tokio::runtime::Handle,
        id: ExprId,
        v: &Value,
    ) -> Result<bool> {
        static DBG_ENTRY: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = DBG_ENTRY.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 5 {
            eprintln!(
                "DBG handle_update: id={id:?} window_ref={:?} title={:?} size={:?} theme={:?} content_ref={:?} v={v:?}",
                self.window_ref.id, self.title.r.id, self.size.r.id, self.theme.r.id, self.content_ref.id
            );
        }
        if id == self.window_ref.id {
            self.window_ref.last = Some(v.clone());
            let resolved = rt
                .block_on(ResolvedWindow::compile(self.gx.clone(), v.clone()))
                .context("window ref recompile")?;
            self.title = resolved.title;
            self.size = resolved.size;
            self.theme = resolved.theme;
            self.content_ref = resolved.content_ref;
            self.content = resolved.content;
            self.needs_redraw = true;
            return Ok(true);
        }
        let mut changed = false;
        if self.title.update(id, v).context("surface update title")?.is_some() {
            changed = true;
        }
        if self.size.update(id, v).context("surface update size")?.is_some() {
            // TODO: resize texture/staging buffer when size changes
            changed = true;
        }
        if self.theme.update(id, v).context("surface update theme")?.is_some() {
            changed = true;
        }
        if id == self.content_ref.id {
            self.content_ref.last = Some(v.clone());
            self.content = rt
                .block_on(compile(self.gx.clone(), v.clone()))
                .context("surface content recompile")?;
            changed = true;
        }
        let content_changed = self.content.handle_update(rt, id, v)?;
        changed |= content_changed;
        {
            static DBG_UPD: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = DBG_UPD.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 20 {
                eprintln!(
                    "DBG handle_update: id={id:?} changed={changed} content_changed={content_changed}",
                );
            }
        }
        self.needs_redraw |= changed;
        Ok(changed)
    }

    pub fn push_event(&mut self, event: iced_core::Event) {
        self.pending_events.push(event);
        self.needs_redraw = true;
    }

    pub fn editor_action(
        &mut self,
        id: ExprId,
        action: &iced_widget::text_editor::Action,
    ) -> Option<(graphix_rt::CallableId, Value)> {
        self.content.editor_action(id, action)
    }
}
