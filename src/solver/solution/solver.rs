//! Contrato común para las estrategias de solución del JSP.

use crate::domain::{Jsp, solution::Schedule};
use crate::solver::stop::StopHandle;

/// Interfaz genérica para los algoritmos de resolución del JSP.
pub trait Solver {
    /// Resuelve el problema con un criterio de detención.
    ///
    /// Cuando se solicita la detención, retorna la mejor solución encontrada
    /// hasta ese momento.
    fn solve_with_stop(&self, jsp: &Jsp, stop: &mut StopHandle) -> Option<Schedule>;

    /// Resuelve el problema sin condiciones de parada externas.
    ///
    /// Retorna `None` si la estrategia no produce una solución.
    fn solve(&self, jsp: &Jsp) -> Option<Schedule> {
        // Un manejador vacío no detiene la ejecución por sí mismo.
        let mut stop = StopHandle::new();
        self.solve_with_stop(jsp, &mut stop)
    }
}
