use std::time::{Duration, Instant};

use super::StopCondition;

/// Condición de parada basada en tiempo límite de ejecución.
pub struct TimeoutCondition {
    duration: Duration,
    start_time: Instant,
}

impl TimeoutCondition {
    /// Crea una nueva condición de parada por tiempo límite.
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            start_time: Instant::now(),
        }
    }
}

impl StopCondition for TimeoutCondition {
    /// Evalúa si el tiempo transcurrido desde `start_time` es mayor o igual al límite.
    fn should_stop(&self) -> bool {
        self.start_time.elapsed() >= self.duration
    }

    /// Reinicia el temporizador estableciendo `start_time` al instante actual.
    fn reset(&mut self) {
        self.start_time = Instant::now();
    }
}
