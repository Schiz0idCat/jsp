//! Contrato común para las estrategias de solución del JSP.

use crate::domain::{Jsp, solution::Schedule};
use crate::solver::stop::SearchControl;

/// Interfaz genérica para los algoritmos de resolución del JSP.
pub trait Solver {
    /// Resuelve el problema con un control de parada y presupuesto.
    ///
    /// Cuando se solicita la detención, retorna la mejor solución encontrada
    /// hasta ese momento.
    fn solve_with_stop(&self, jsp: &Jsp, control: &mut SearchControl) -> Option<Schedule>;

    /// Resuelve el problema sin límites configurados.
    ///
    /// Retorna `None` si la estrategia no produce una solución.
    fn solve(&self, jsp: &Jsp) -> Option<Schedule> {
        // Sin condiciones externas ni presupuesto, la estrategia decide cuándo
        // termina por su propia lógica.
        let mut control = SearchControl::default();
        self.solve_with_stop(jsp, &mut control)
    }
}
