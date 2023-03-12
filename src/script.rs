pub trait Script: Send + Sync {
    fn update(&self) {}
}