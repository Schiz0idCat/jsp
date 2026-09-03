use crate::domain::{Jsp, solution::Schedule};
use crate::solver::stop::StopHandle;

/// Interfaz genérica para algorimos de resolución del JSP.
pub trait Solver {
    /// Resuelve el problema con un criterio de detención.
    /// Cuando se detiene, retorna la mejor solución encontrada para ese momento.
    fn solve_with_stop(&self, jsp: &Jsp, stop: &mut StopHandle) -> Option<Schedule>;

    /// Resuelve el problema.
    fn solve(&self, jsp: &Jsp) -> Option<Schedule> {
        let mut stop = StopHandle::new();
        self.solve_with_stop(jsp, &mut stop)
    }
}
