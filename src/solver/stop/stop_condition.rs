pub trait StopCondition: Send + Sync {
    fn should_stop(&self) -> bool;
    fn reset(&mut self) {}
}
