use futures::executor::block_on;
use log::info;
use winit::window::Window;

pub mod logger;

pub fn create_adapter(instance: &wgpu::Instance, surface: &wgpu::Surface) -> wgpu::Adapter {
    block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false
    })).unwrap();
    let adapter = instance.enumerate_adapters(wgpu::Backends::all()).next().unwrap();
    info!("adapter: {:?}", adapter.get_info());
    adapter
}

pub fn create_device_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
            limits: wgpu::Limits::default(),
            label: None
        },
        None
    )).unwrap()
}

pub fn configure_surface(
    window: &Window,
    device: &wgpu::Device,
    adapter: &wgpu::Adapter,
    surface: &wgpu::Surface,
    vsync: bool
) -> wgpu::SurfaceConfiguration {
    let size = window.inner_size();
    let surface_caps = surface.get_capabilities(&adapter);
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_caps.formats.iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]),
        width: size.width,
        height: size.height,
        present_mode: if vsync {
            wgpu::PresentMode::AutoVsync
        } else {
            wgpu::PresentMode::AutoNoVsync
        },
        alpha_mode: wgpu::CompositeAlphaMode::Opaque,
        view_formats: vec![]
    };
    surface.configure(device, &config);
    config
}