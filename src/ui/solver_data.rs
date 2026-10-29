use super::Strategy;
use crate::domain::Jsp;
use crate::domain::solution::Schedule;
use crate::solver::solution::Solver;
use crate::solver::solution::strategies::{
    BruteForce, LocalSearch, Random, SimulatedAnnealing, Swap,
};
use crate::solver::stop::{CancelCondition, StopHandle, TimeoutCondition};
use crate::ui::errors::SolverError;

use std::path::Path;
use std::time::Duration;

/// Encapsula una instacia [`Jsp`], sus condiciones de parada y estrategia para solucionarlo.
pub struct SolverData {
    jsp: Jsp,
    solver: Box<dyn Solver>,
    stop: StopHandle,
    cancel: CancelCondition,
}

impl SolverData {
    /// Inicializa y valida los parámetros para resolver el problema.
    pub fn new(
        filepath: &Path,
        timeout: Option<Duration>,
        strategy: Strategy,
    ) -> Result<Self, SolverError> {
        let jsp = Jsp::from_file(filepath)?;

        let cancel = CancelCondition::handle_ctrlc()?;

        let mut stop = StopHandle::new().with_condition(cancel.clone());

        if let Some(time) = timeout {
            stop = stop.with_condition(TimeoutCondition::new(time));
        }

        let solver: Box<dyn Solver> = match strategy {
            Strategy::Bruteforce => Box::new(BruteForce::new()),
            Strategy::Random { samples } => Box::new(Random::new(samples)),
            Strategy::LocalSearch { samples } => Box::new(LocalSearch::new(samples)),
            Strategy::Swap => Box::new(Swap::new()),
            Strategy::SimulatedAnnealing {
                t_initial,
                t_min,
                alpha,
            } => Box::new(SimulatedAnnealing::new(t_initial, t_min, alpha)),
        };

        Ok(Self {
            jsp,
            solver,
            stop,
            cancel,
        })
    }
}

impl SolverData {
    /// Returns a reference to the JSP instance.
    pub fn jsp(&self) -> &Jsp {
        &self.jsp
    }

    /// Returns a reference to the cancellation handle for progress tracking.
    pub fn cancel_handle(&self) -> &CancelCondition {
        &self.cancel
    }

    /// Executes the selected solving strategy.
    pub fn solve(&mut self) -> Option<Schedule> {
        self.solver.solve_with_stop(&self.jsp, &mut self.stop)
    }
}
