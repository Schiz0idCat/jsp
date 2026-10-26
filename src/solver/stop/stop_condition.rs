/// Define el comportamiento genérico de parada para los solucionadores.
pub trait StopCondition: Send + Sync {
    /// Evalua si se debe detener la ejecución.
    fn should_stop(&self) -> bool;

    /// Permite resetear la condición de parada.
    fn reset(&mut self) {}
}
