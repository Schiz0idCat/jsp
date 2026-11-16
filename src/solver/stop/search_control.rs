//! Control combinado de condiciones externas y presupuesto de búsqueda.

use super::{SearchBudget, StopHandle};

/// Combina las condiciones de parada con el presupuesto de iteraciones.
///
/// Las estrategias solo necesitan llamar a [`SearchControl::tick`] una vez por
/// iteración. El control se encarga de consultar las condiciones externas y de
/// llevar el conteo de iteraciones.
pub struct SearchControl {
    /// Condiciones externas de parada, como tiempo límite o cancelación.
    stop: StopHandle,

    /// Política de iteraciones de la búsqueda.
    budget: SearchBudget,

    /// Cantidad de iteraciones ejecutadas desde el último [`SearchControl::start`].
    iterations: u64,
}

impl SearchControl {
    /// Crea un control a partir de las condiciones y el presupuesto.
    ///
    /// El contador de iteraciones siempre comienza en cero.
    pub fn new(stop: StopHandle, budget: SearchBudget) -> Self {
        Self {
            stop,
            budget,
            iterations: 0,
        }
    }

    /// Reinicia las condiciones externas y el contador de iteraciones.
    pub fn start(&mut self) {
        self.stop.start();
        self.iterations = 0;
    }

    /// Indica si alguna condición o el presupuesto piden detener la búsqueda.
    pub fn should_stop(&self) -> bool {
        !self.budget.allows(self.iterations) || self.stop.should_stop()
    }

    /// Registra una iteración y retorna `true` si debe detenerse la búsqueda.
    ///
    /// Primero se consulta el estado actual. Si no hay una parada pendiente, se
    /// incrementa el contador. De esta forma se ejecutan exactamente `N`
    /// iteraciones cuando el presupuesto tiene límite `N`.
    pub fn tick(&mut self) -> bool {
        if self.should_stop() {
            return true;
        }

        self.iterations += 1;
        false
    }

    /// Retorna la cantidad de iteraciones ejecutadas.
    pub fn iterations(&self) -> u64 {
        self.iterations
    }
}

impl Default for SearchControl {
    fn default() -> Self {
        Self::new(StopHandle::new(), SearchBudget::default())
    }
}
