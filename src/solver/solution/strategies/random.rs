//! Esta estrategia resuelve el taller 5

use rand::rng;
use rand::seq::SliceRandom;

use super::super::SolutionBuilder;
use super::super::Solver;
use crate::domain::Jsp;
use crate::domain::solution::Schedule;
use crate::solver::stop::StopHandle;

/// Estrategia de generación y evaluación de soluciones aleatorias.
pub struct Random {
    samples: usize,
}

impl Random {
    pub fn new(samples: usize) -> Self {
        Self { samples }
    }
}

impl SolutionBuilder for Random {}

impl Solver for Random {
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

        let mut base_sequence = Vec::new();
        for (idx, job) in jsp.jobs().iter().enumerate() {
            for _ in 0..job.operations().len() {
                base_sequence.push(idx);
            }
        }

        let mut best_makespan = usize::MAX;
        let mut best_sequence: Option<Vec<usize>> = None;
        let mut rng = rng();
        let mut current_seq = base_sequence;

        stop.start();

        for _ in 0..self.samples {
            if stop.should_stop() {
                break;
            }

            current_seq.shuffle(&mut rng);

            let makespan = self.evaluate_makespan(jsp, &current_seq, max_machine_id);

            if makespan < best_makespan {
                best_makespan = makespan;
                best_sequence = Some(current_seq.clone());
            }
        }

        let best_seq = best_sequence?;
        Some(self.build_schedule(jsp, &best_seq, max_machine_id))
    }
}
