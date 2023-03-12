mod first_person;

pub use first_person::*;
use winit::dpi::PhysicalSize;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraBinding {
    pub position: [f32;4],
    pub direction: [f32;4],
    pub screen_size: [f32;4]
}

#[allow(unused)]
pub trait Camera: Send + Sync {
    fn get_bind_group(&self) -> &wgpu::BindGroup;
    fn resize(&self, new_size: PhysicalSize<u32>) {}
    fn update(&self) {}
}