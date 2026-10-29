//! Estrategia Simulated Annealing.
//!
//! El código fue realizado integramente por los integrantes del grupo.
//! Pero la metaheurística fue inventada por Kirkpatrick et al.
//!
//! Referencia:
//! Kirkpatrick, S.; Gelatt, C. D.; Vecchi, M. P. (1983).
//! «Optimization by Simulated Annealing». Science (en inglés) 220 (4598): 671-680.

use crate::domain::{Jsp, solution::Schedule};
use crate::solver::solution::{Solver, solution_builder::SolutionBuilder};
use crate::solver::stop::StopHandle;

use rand::seq::SliceRandom;
use rand::{RngExt, rng};

pub struct SimulatedAnnealing {
    /// Aleatoriedad. Dispocisión para aceptar soluciones peores.
    t_initial: f64,

    /// Punto en el que deja de buscar soluciones.
    t_min: f64,

    /// Tasa de enfriamiento.
    alpha: f64,
}

impl SimulatedAnnealing {
    pub fn new(t_initial: f64, t_min: f64, alpha: f64) -> Self {
        Self {
            t_initial,
            t_min,
            alpha,
        }
    }
}

impl SolutionBuilder for SimulatedAnnealing {}

impl Solver for SimulatedAnnealing {
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

        // Solución inicial aleatoria
        let mut current_seq: Vec<usize> = jsp
            .jobs()
            .iter()
            .enumerate()
            .flat_map(|(idx, job)| std::iter::repeat_n(idx, job.operations().len()))
            .collect();

        let mut rng = rng();
        current_seq.shuffle(&mut rng);

        let mut current_makespan = self.evaluate_makespan(jsp, &current_seq, max_machine_id);

        let mut best_seq = current_seq.clone();
        let mut best_makespan = current_makespan;

        let mut t = self.t_initial;
        stop.start();

        while t > self.t_min && !stop.should_stop() {
            // Tomar una posición aleatorea
            let i = rng.random_range(0..current_seq.len());
            let mut j = rng.random_range(0..current_seq.len());

            while i == j || current_seq[i] == current_seq[j] {
                j = rng.random_range(0..current_seq.len());
            }

            // Generar vecino
            current_seq.swap(i, j);
            let neighbor_makespan = self.evaluate_makespan(jsp, &current_seq, max_machine_id);

            let delta = neighbor_makespan as f64 - current_makespan as f64;

            // Criterio de aceptación Metropolis
            if delta < 0.0 || rng.random::<f64>() < (-delta / t).exp() {
                // Se acepta el movimiento
                current_makespan = neighbor_makespan;

                if current_makespan < best_makespan {
                    best_makespan = current_makespan;
                    best_seq = current_seq.clone();
                }
            } else {
                // Se deshace el movimiento
                current_seq.swap(i, j);
            }

            // Enfriamiento
            t *= self.alpha;
        }

        Some(self.build_schedule(jsp, &best_seq, max_machine_id))
    }
}
