//! Condición de parada activada por una cancelación externa.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use super::StopCondition;

/// Condición de parada basada en una señal de cancelación externa.
///
/// Se utiliza para gestionar la interrupción del usuario con `Ctrl+C`.
#[derive(Clone)]
pub struct CancelCondition {
    /// Bandera compartida entre el manejador de la señal y el solucionador.
    token: Arc<AtomicBool>,
}

// Inicializadores
impl CancelCondition {
    /// Crea una condición a partir de una bandera compartida.
    pub fn new(token: Arc<AtomicBool>) -> Self {
        Self { token }
    }
}

impl CancelCondition {
    /// Registra un manejador de `Ctrl+C` que activa la cancelación.
    ///
    /// # Errors
    ///
    /// Retorna [`ctrlc::Error`] si no se puede instalar el manejador de la señal.
    pub fn handle_ctrlc() -> Result<Self, ctrlc::Error> {
        let token = Arc::new(AtomicBool::new(false));
        let token_clone = Arc::clone(&token);

        ctrlc::set_handler(move || {
            // La bandera se comparte con el hilo que ejecuta la estrategia.
            token_clone.store(true, Ordering::Relaxed);
        })?;

        Ok(Self { token })
    }

    /// Indica si la cancelación fue activada.
    pub fn is_cancelled(&self) -> bool {
        self.token.load(Ordering::Relaxed)
    }
}

impl StopCondition for CancelCondition {
    /// Evalúa si se emitió una señal de cancelación.
    fn should_stop(&self) -> bool {
        self.token.load(Ordering::Relaxed)
    }
}
