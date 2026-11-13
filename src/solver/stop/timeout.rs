//! Condición de parada basada en un tiempo límite.

use std::time::{Duration, Instant};

use super::StopCondition;

/// Condición de parada basada en el tiempo transcurrido desde el inicio.
pub struct TimeoutCondition {
    /// Tiempo máximo permitido.
    duration: Duration,

    /// Instante en que comienza a contarse el tiempo.
    start_time: Instant,
}

impl TimeoutCondition {
    /// Crea una condición de parada con el tiempo máximo indicado.
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            start_time: Instant::now(),
        }
    }
}

impl StopCondition for TimeoutCondition {
    /// Evalúa si el tiempo transcurrido alcanzó el límite.
    fn should_stop(&self) -> bool {
        self.start_time.elapsed() >= self.duration
    }

    /// Reinicia el temporizador al instante actual.
    fn reset(&mut self) {
        self.start_time = Instant::now();
    }
}
