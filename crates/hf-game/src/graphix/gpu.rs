use iced_wgpu::wgpu;

/// Cloned wgpu state from Bevy's render world, used to create iced renderers
/// on the same GPU device. All wgpu handles are internally reference-counted,
/// so clones are cheap.
pub struct IcedGpu {
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub format: wgpu::TextureFormat,
}
