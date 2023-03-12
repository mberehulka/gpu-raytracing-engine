use winit::{window::{Window, WindowBuilder}, event_loop::EventLoop};

pub fn new_window(event_loop: &EventLoop<()>) -> Window {
    match WindowBuilder::new()
        .with_theme(Some(winit::window::Theme::Dark))
        .build(event_loop)
    {
        Ok(v) => v,
        Err(e) => panic!("Error creating the window: {e}")
    }
}