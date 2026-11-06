// Esto no sigue el patrón factory, ni respeta el principio abierto-cerrado.
// Pero funciona.

use super::Strategy;
use crate::solver::RandomSeed;
use crate::solver::solution::Solver;
use crate::solver::solution::strategies::{
    BruteForce, LocalSearch, Random, SimulatedAnnealing, Swap,
};

/// Construye la estrategia indicada por la CLI.
///
/// La semilla se entrega únicamente a las estrategias que utilizan
/// aleatoriedad; las estrategias deterministas no la reciben.
pub fn build(strategy: Strategy, seed: RandomSeed) -> Box<dyn Solver> {
    match strategy {
        Strategy::Bruteforce => Box::new(BruteForce::new()),
        Strategy::Random { samples } => Box::new(Random::new(samples, seed)),
        Strategy::LocalSearch { samples } => Box::new(LocalSearch::new(samples, seed)),
        Strategy::Swap => Box::new(Swap::new(seed)),
        Strategy::SimulatedAnnealing {
            t_initial,
            t_min,
            alpha,
        } => Box::new(SimulatedAnnealing::new(t_initial, t_min, alpha, seed)),
    }
}
