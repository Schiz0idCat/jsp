//! Estrategia de muestreo aleatorio de secuencias.

use rand::seq::SliceRandom;

use super::super::SolutionBuilder;
use super::super::Solver;
use crate::domain::Jsp;
use crate::domain::solution::Schedule;
use crate::solver::RandomSeed;
use crate::solver::stop::StopHandle;

/// Estrategia de generación y evaluación de soluciones aleatorias.
pub struct Random {
    /// Cantidad de secuencias aleatorias que se evalúan.
    samples: usize,

    /// Semilla del generador aleatorio.
    seed: RandomSeed,
}

impl Random {
    /// Crea la estrategia con una cantidad de muestras y una semilla.
    pub fn new(samples: usize, seed: RandomSeed) -> Self {
        Self { samples, seed }
    }
}

impl SolutionBuilder for Random {}

impl Solver for Random {
    /// Genera y evalúa secuencias aleatorias, conservando la mejor.
    fn solve_with_stop(&self, jsp: &Jsp, stop: &mut StopHandle) -> Option<Schedule> {
        if jsp.jobs().is_empty() {
            return None;
        }

        // Obtiene la máquina de mayor identificador para dimensionar los
        // arreglos internos de evaluación.
        let max_machine_id = jsp
            .jobs()
            .iter()
            .flat_map(|job| job.operations().iter())
            .map(|op| op.machine())
            .max()?;

        // Secuencia base: cada trabajo aparece una vez por operación.
        let mut base_sequence = Vec::new();
        for (idx, job) in jsp.jobs().iter().enumerate() {
            for _ in 0..job.operations().len() {
                base_sequence.push(idx);
            }
        }

        let mut best_makespan = usize::MAX;
        let mut best_sequence: Option<Vec<usize>> = None;
        let mut rng = self.seed.rng();
        let mut current_seq = base_sequence;

        stop.start();

        for _ in 0..self.samples {
            if stop.should_stop() {
                break;
            }

            // Mezcla la secuencia para obtener una solución factible aleatoria.
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
