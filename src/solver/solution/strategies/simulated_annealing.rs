//! Estrategia Simulated Annealing.

use crate::domain::{Jsp, solution::Schedule};
use crate::solver::RandomSeed;
use crate::solver::solution::{Solver, solution_builder::SolutionBuilder};
use crate::solver::stop::StopHandle;

use rand::RngExt;
use rand::seq::SliceRandom;

/// Estrategia de solución basada en Simulated Annealing.
pub struct SimulatedAnnealing {
    /// Temperatura inicial: controla la probabilidad de aceptar soluciones peores.
    t_initial: f64,

    /// Temperatura mínima: punto en el que termina la búsqueda.
    t_min: f64,

    /// Tasa de enfriamiento aplicada en cada iteración.
    alpha: f64,

    /// Semilla del generador aleatorio.
    seed: RandomSeed,
}

impl SimulatedAnnealing {
    /// Crea la estrategia con sus parámetros de temperatura y su semilla.
    pub fn new(t_initial: f64, t_min: f64, alpha: f64, seed: RandomSeed) -> Self {
        Self {
            t_initial,
            t_min,
            alpha,
            seed,
        }
    }
}

impl SolutionBuilder for SimulatedAnnealing {}

impl Solver for SimulatedAnnealing {
    /// Ejecuta el ciclo de enfriamiento y retorna la mejor solución encontrada.
    fn solve_with_stop(&self, jsp: &Jsp, stop: &mut StopHandle) -> Option<Schedule> {
        if jsp.jobs().is_empty() {
            return None;
        }

        let max_machine_id = jsp
            .jobs()
            .iter()
            .flat_map(|job| job.operations().iter())
            .map(|op| op.machine())
            .max()?;

        // Construye una secuencia inicial aleatoria con una aparición por operación.
        let mut current_seq: Vec<usize> = jsp
            .jobs()
            .iter()
            .enumerate()
            .flat_map(|(idx, job)| std::iter::repeat_n(idx, job.operations().len()))
            .collect();

        let mut rng = self.seed.rng();
        current_seq.shuffle(&mut rng);

        let mut current_makespan = self.evaluate_makespan(jsp, &current_seq, max_machine_id);

        let mut best_seq = current_seq.clone();
        let mut best_makespan = current_makespan;

        let mut t = self.t_initial;
        stop.start();

        while t > self.t_min && !stop.should_stop() {
            // Selecciona dos posiciones distintas que contengan trabajos distintos.
            let i = rng.random_range(0..current_seq.len());
            let mut j = rng.random_range(0..current_seq.len());

            while i == j || current_seq[i] == current_seq[j] {
                j = rng.random_range(0..current_seq.len());
            }

            // Genera el vecino intercambiando ambas posiciones.
            current_seq.swap(i, j);
            let neighbor_makespan = self.evaluate_makespan(jsp, &current_seq, max_machine_id);

            let delta = neighbor_makespan as f64 - current_makespan as f64;

            // Criterio de aceptación de Metropolis.
            if delta < 0.0 || rng.random::<f64>() < (-delta / t).exp() {
                // Se acepta el movimiento.
                current_makespan = neighbor_makespan;

                if current_makespan < best_makespan {
                    best_makespan = current_makespan;
                    best_seq = current_seq.clone();
                }
            } else {
                // Se deshace el movimiento.
                current_seq.swap(i, j);
            }

            // Enfriamiento geométrico.
            t *= self.alpha;
        }

        Some(self.build_schedule(jsp, &best_seq, max_machine_id))
    }
}
