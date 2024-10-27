pub trait LifecycleHandler {
    fn initialize(&mut self) {}
    fn update(&mut self) {}
    fn shutdown(&mut self) {}
}
