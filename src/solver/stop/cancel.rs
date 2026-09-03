use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use super::StopCondition;

pub struct CancelTokenCondition {
    token: Arc<AtomicBool>,
}

impl CancelTokenCondition {
    pub fn new(token: Arc<AtomicBool>) -> Self {
        Self { token }
    }
}

impl StopCondition for CancelTokenCondition {
    fn should_stop(&self) -> bool {
        self.token.load(Ordering::Relaxed)
    }
}
