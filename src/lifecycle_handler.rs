use crate::Input;

pub trait LifecycleHandler {
    fn initialize(&mut self);
    fn input(&mut self, input: &Input);
    fn fixed_update(&mut self, delta: f32);
    fn update(&mut self, delta: f32);
    fn shutdown(&mut self);
}
