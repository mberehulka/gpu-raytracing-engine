use std::sync::Arc;

use winit::{event_loop::{EventLoop, ControlFlow}, platform::run_return::EventLoopExtRunReturn,
    event::{Event, WindowEvent, KeyboardInput, ElementState}};

use crate::{Context, Threads};

pub struct Engine {
    event_loop: Option<EventLoop<()>>,
    threads: Arc<Threads>,
    pub context: Context
}
impl Engine {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let context = Context::new(&event_loop);
        let threads = Threads::new().into();

        Self {
            event_loop: Some(event_loop),
            threads,
            context
        }
    }
    pub fn start(mut self) {
        self.event_loop.take().unwrap().run_return(|event, _, control_flow| {
            if self.context.get_close_request() {
                return *control_flow = ControlFlow::Exit
            }
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(key), state: ElementState::Pressed, .. }, .. } =>
                        self.context.key_pressed(key),
                    WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(key), state: ElementState::Released, .. }, .. } =>
                        self.context.key_released(key),
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(new_size) => self.context.resize(new_size),
                    _ => {}
                },
                Event::MainEventsCleared => self.context.window.request_redraw(),
                Event::RedrawRequested(_) => {
                    for script in self.context.scripts.lock().unwrap().iter() {
                        self.threads.send(crate::Job::Update(script.clone()))
                    }
                    {self.threads.send(crate::Job::UpdateCamera(self.context.camera.lock().unwrap().clone()));}
                    self.threads.wait();
                    self.context.draw();
                },
                _ => {}
            }
        });
    }
}