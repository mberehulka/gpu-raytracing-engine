use std::{collections::HashSet, sync::{atomic::AtomicBool, Mutex, Arc}};

use wgpu::{Surface, SurfaceConfiguration, Device, Queue};
use winit::{window::Window, event_loop::EventLoop, event::VirtualKeyCode, dpi::PhysicalSize};

use crate::{new_window, utils, MainShader, Script, Camera, FirstPersonCamera};

#[derive(Clone)]
pub struct Context {
    pub surface: Arc<Surface>,
    surface_config: Arc<Mutex<SurfaceConfiguration>>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,

    close_request: Arc<AtomicBool>,
    keys_pressed: Arc<Mutex<HashSet<VirtualKeyCode>>>,
    pub scripts: Arc<Mutex<Vec<Arc<Box<dyn Script>>>>>,
    pub window: Arc<Window>,

    main_shader: Arc<MainShader>,

    pub camera: Arc<Mutex<Arc<Box<dyn Camera>>>>
}
impl Context {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = new_window(event_loop);
        
        let instance = wgpu::Instance::new(Default::default());
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = utils::create_adapter(&instance, &surface);
        let (device, queue) = utils::create_device_queue(&adapter);
        let queue = Arc::new(queue);
        
        let surface_config = utils::configure_surface(&window, &device, &adapter, &surface, true);

        let main_shader = MainShader::new(&device, surface_config.format);
        let camera = FirstPersonCamera::new(queue.clone(), &device, &main_shader, &window);

        Self {
            surface: surface.into(),
            surface_config: Arc::new(Mutex::new(surface_config)),
            device: device.into(),
            queue,
            
            close_request: AtomicBool::new(false).into(),
            keys_pressed: Default::default(),
            scripts: Default::default(),
            window: window.into(),

            main_shader: main_shader.into(),

            camera: Arc::new(Mutex::new(Arc::new(Box::new(camera))))
        }
    }

    pub fn close(&self) {
        self.close_request.store(true, std::sync::atomic::Ordering::Relaxed)
    }
    pub fn get_close_request(&self) -> bool {
        self.close_request.load(std::sync::atomic::Ordering::Relaxed)
    }
    
    pub fn key_pressed(&self, key: VirtualKeyCode) {
        self.keys_pressed.lock().unwrap().insert(key);
    }
    pub fn key_released(&self, key: VirtualKeyCode) {
        self.keys_pressed.lock().unwrap().remove(&key);
    }
    pub fn get_key(&self, key: VirtualKeyCode) -> Option<VirtualKeyCode> {
        self.keys_pressed.lock().unwrap().get(&key).copied()
    }
    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        self.get_key(key).is_some()
    }
    
    pub fn set_vsync(&self, vsync: bool) {
        let mut surface_config = self.surface_config.lock().unwrap();
        surface_config.present_mode = if vsync {
            wgpu::PresentMode::AutoVsync
        } else {
            wgpu::PresentMode::AutoNoVsync
        };
        self.surface.configure(&self.device, &surface_config);
    }

    pub fn add_script<S: Script + 'static>(&self, script: S) -> Arc<Box<dyn Script>> {
        let script: Arc<Box<dyn Script>> = Arc::new(Box::new(script));
        self.scripts.lock().unwrap().push(script.clone());
        script
    }

    pub fn resize(&self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 { return }
        let mut surface_config = self.surface_config.lock().unwrap();
        surface_config.width = new_size.width;
        surface_config.height = new_size.height;
        self.surface.configure(&self.device, &surface_config);
        self.camera.lock().unwrap().resize(new_size)
    }
    pub fn draw(&self) {
        self.main_shader.draw(&self);
    }
}