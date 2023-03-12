use gpu_raytracing_engine::{Engine, Script, Context, Key};

struct MainScript {
    c: Context
}
impl Script for MainScript {
    fn update(&self) {
        if self.c.is_key_pressed(Key::Escape) {
            self.c.close()
        }
    }
}

fn main() {
    gpu_raytracing_engine::utils::logger::start();
    let engine = Engine::new();
    engine.context.add_script(MainScript {
        c: engine.context.clone()
    });
    engine.start();
}