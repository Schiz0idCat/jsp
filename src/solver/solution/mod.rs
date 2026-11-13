//! Estrategias de solución y contrato común de los solucionadores.

mod solution_builder;
mod solver;

/// Implementaciones concretas de estrategias.
pub mod strategies;

use solution_builder::SolutionBuilder;
pub use solver::Solver;
