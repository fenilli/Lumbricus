use crate::ApplicationContext;

pub trait LifecycleHandler {
    fn initialize(&mut self, context: &ApplicationContext);
    fn fixed_update(&mut self, delta: f32);
    fn update(&mut self, delta: f32);
    fn shutdown(&mut self, context: &ApplicationContext);
}
