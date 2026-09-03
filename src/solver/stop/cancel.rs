use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use super::StopCondition;

/// Condición de parada basada en la instrucción de cancelación por parte de un hilo externo
/// (usada para gestionar la interrupción de usuario 'ctrl + c').
#[derive(Clone)]
pub struct CancelCondition {
    token: Arc<AtomicBool>,
}

// Inicializadores
impl CancelCondition {
    pub fn new(token: Arc<AtomicBool>) -> Self {
        Self { token }
    }
}

impl CancelCondition {
    /// Gestiona el evento ctrl + c
    pub fn handle_ctrlc() -> Result<Self, ctrlc::Error> {
        let token = Arc::new(AtomicBool::new(false));
        let token_clone = Arc::clone(&token);

        ctrlc::set_handler(move || {
            token_clone.store(true, Ordering::Relaxed);
        })?;

        Ok(Self { token })
    }

    /// Permite consultar si la cancelación fue activada.
    pub fn is_cancelled(&self) -> bool {
        self.token.load(Ordering::Relaxed)
    }
}

impl StopCondition for CancelCondition {
    /// Evalua si se ha emitido una señal de cancelación
    fn should_stop(&self) -> bool {
        self.token.load(Ordering::Relaxed)
    }
}
