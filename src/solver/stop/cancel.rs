use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use super::StopCondition;

/// Condición de parada basada en la instrucción de cancelación por parte de un hilo externo
/// (usada para gestionar la interrupción de usuario 'ctrl + c').
pub struct CancelTokenCondition {
    token: Arc<AtomicBool>,
}

// Inicializadores
impl CancelTokenCondition {
    pub fn new(token: Arc<AtomicBool>) -> Self {
        Self { token }
    }
}

impl StopCondition for CancelTokenCondition {
    /// Evalua si se ha emitido una señal de cancelación
    fn should_stop(&self) -> bool {
        self.token.load(Ordering::Relaxed)
    }
}
