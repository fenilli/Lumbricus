use crate::ApplicationContext;

pub trait LifecycleHandler {
    fn initialize(&mut self, context: &mut ApplicationContext);
    fn fixed_update(&mut self, delta: f32, context: &ApplicationContext);
    fn update(&mut self, delta: f32, context: &ApplicationContext);
    fn shutdown(&mut self, context: &mut ApplicationContext);
}
