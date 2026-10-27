//! Esta estrategia resuelve el taller 6

use crate::domain::{Jsp, solution::Schedule};
use crate::solver::solution::{Solver, solution_builder::SolutionBuilder};
use crate::solver::stop::StopHandle;

/// Estrategia de generación y evaluación de soluciones aleatorias.
pub struct Swap;

impl Swap {
    pub fn new() -> Self {
        todo!()
    }
}

impl SolutionBuilder for Swap {}

impl Solver for Swap {
    fn solve_with_stop(&self, jsp: &Jsp, stop: &mut StopHandle) -> Option<Schedule> {
        todo!()
    }
}
