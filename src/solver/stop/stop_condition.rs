//! Contrato común de las condiciones de parada.

/// Comportamiento genérico de parada para los solucionadores.
pub trait StopCondition: Send + Sync {
    /// Evalúa si se debe detener la ejecución.
    fn should_stop(&self) -> bool;

    /// Reinicia el estado interno de la condición de parada.
    ///
    /// La implementación por defecto no hace nada.
    fn reset(&mut self) {}
}
