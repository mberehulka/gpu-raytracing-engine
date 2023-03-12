use std::sync::{Mutex, atomic::AtomicBool, Arc};

use cgmath::{Vector3, Vector4, Quaternion, Rotation3, Deg};
use wgpu::{Device, util::DeviceExt, Queue};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{MainShader, CameraBinding, Camera};

pub struct FirstPersonCamera {
    queue: Arc<Queue>,
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    position: Mutex<Vector3<f32>>,
    rotation: Mutex<Vector3<f32>>,
    screen_size: Mutex<PhysicalSize<u32>>,
    needs_update: AtomicBool
}
impl FirstPersonCamera {
    pub fn new(queue: Arc<Queue>, device: &Device, main_shader: &MainShader, window: &Window) -> Self {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[ CameraBinding {
                    direction: [0.;4],
                    position: [0.;4],
                    screen_size: [0.;4]
                } ]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );
        Self {
            queue,
            bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &main_shader.get_camera_bgl(),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding()
                    }
                ]
            }),
            buffer,
            position: Mutex::new([0.;3].into()),
            rotation: Mutex::new([0.;3].into()),
            screen_size: Mutex::new(window.inner_size()),
            needs_update: AtomicBool::new(true)
        }
    }

    pub fn get_position(&self) -> Vector3<f32> {
        *self.position.lock().unwrap()
    }
    pub fn set_position(&self, v: Vector3<f32>) {
        *self.position.lock().unwrap() = v;
        self.needs_update.store(true, std::sync::atomic::Ordering::Relaxed)
    }
    pub fn translate(&self, v: Vector3<f32>) {
        *self.position.lock().unwrap() += v;
        self.needs_update.store(true, std::sync::atomic::Ordering::Relaxed)
    }

    pub fn get_rotation(&self) -> Vector3<f32> {
        *self.rotation.lock().unwrap()
    }
    /// 0° < x, y, z < 360°
    pub fn set_rotation(&self, v: Vector3<f32>) {
        *self.rotation.lock().unwrap() = v;
        self.needs_update.store(true, std::sync::atomic::Ordering::Relaxed)
    }
    /// 0° < x, y, z < 360°
    pub fn rotate(&self, v: Vector3<f32>) {
        *self.rotation.lock().unwrap() += v;
        self.needs_update.store(true, std::sync::atomic::Ordering::Relaxed)
    }
    
    pub fn get_binding(&self) -> CameraBinding {
        let size = *self.screen_size.lock().unwrap();
        let rotation = self.get_rotation();
        CameraBinding {
            position: self.get_position().extend(1.).into(),
            direction: (
                Quaternion::from_angle_x(Deg(rotation.x)) *
                Quaternion::from_angle_y(Deg(rotation.y)) *
                Quaternion::from_angle_z(Deg(rotation.z)) *
                Vector3::new(0., 0., 1.)
            ).extend(1.).into(),
            screen_size: Vector4::new(
                if size.width > 0 {
                    (1. / size.width as f32) * 2.
                } else { 0. },
                if size.height > 0 {
                    (1. / size.height as f32) * 2.
                } else { 0. },
                1., 1.
            ).into(),
        }
    }
}
impl Camera for FirstPersonCamera {
    fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
    fn resize(&self, new_size: PhysicalSize<u32>) {
        *self.screen_size.lock().unwrap() = new_size;
        self.needs_update.store(true, std::sync::atomic::Ordering::Relaxed)
    }
    fn update(&self) {
        if self.needs_update.load(std::sync::atomic::Ordering::Relaxed) {
            self.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[ self.get_binding() ]));
            self.needs_update.store(false, std::sync::atomic::Ordering::Relaxed)
        }
    }
}