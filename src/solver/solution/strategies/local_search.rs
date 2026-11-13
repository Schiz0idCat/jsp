//! Estrategia de búsqueda local por primera mejora.

use rand::seq::SliceRandom;

use super::super::SolutionBuilder;
use super::super::Solver;
use crate::domain::Jsp;
use crate::domain::solution::Schedule;
use crate::solver::RandomSeed;
use crate::solver::stop::StopHandle;

/// Estrategia de [`Jsp`] resuelta mediante búsqueda local por primera mejora.
///
/// Genera soluciones iniciales aleatorias y explora iterativamente su
/// vecindario mediante intercambios hasta alcanzar un mínimo local.
pub struct LocalSearch {
    /// Cantidad de soluciones iniciales que se exploran.
    samples: usize,

    /// Semilla del generador aleatorio.
    seed: RandomSeed,
}

impl LocalSearch {
    /// Crea la estrategia con una cantidad de muestras y una semilla.
    pub fn new(samples: usize, seed: RandomSeed) -> Self {
        Self { samples, seed }
    }

    /// Aplica búsqueda local completa con criterio de primera mejora.
    ///
    /// Explora el vecindario de la secuencia mediante intercambios hasta
    /// alcanzar un mínimo local o hasta que se cumpla una condición de parada.
    /// Retorna el makespan de la secuencia resultante.
    fn first_improvement(
        &self,
        jsp: &Jsp,
        sequence: &mut [usize],
        max_machine_id: usize,
        stop: &StopHandle,
        iter_count: &mut u64,
    ) -> usize {
        // Se consulta la condición de parada cada cierto número de evaluaciones.
        const CHECK_INTERVAL: u64 = 10_000;
        let mut current_makespan = self.evaluate_makespan(jsp, sequence, max_machine_id);

        if sequence.len() <= 1 {
            return current_makespan;
        }

        'search: loop {
            let mut improved = false;

            for i in 0..sequence.len() {
                for j in (i + 1)..sequence.len() {
                    if sequence[i] == sequence[j] {
                        continue; // Intercambiar el mismo trabajo no altera la secuencia.
                    }

                    if (*iter_count).is_multiple_of(CHECK_INTERVAL) && stop.should_stop() {
                        break 'search;
                    }
                    *iter_count += 1;

                    sequence.swap(i, j);
                    let makespan = self.evaluate_makespan(jsp, sequence, max_machine_id);

                    // Primera mejora: se acepta el primer vecino que mejore.
                    if makespan < current_makespan {
                        current_makespan = makespan;
                        improved = true;
                        break;
                    } else {
                        sequence.swap(i, j); // Revierte el movimiento si no mejora.
                    }
                }

                if improved {
                    break; // Reinicia la exploración desde la nueva solución.
                }
            }

            if !improved {
                break; // Ningún vecino mejora: se alcanzó un mínimo local.
            }
        }

        current_makespan
    }
}

impl SolutionBuilder for LocalSearch {}

impl Solver for LocalSearch {
    /// Explora varias soluciones iniciales y retorna la mejor encontrada.
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
        let mut iter_count: u64 = 0;

        for _ in 0..self.samples {
            if stop.should_stop() {
                break;
            }

            // Cada muestra parte de una permutación inicial distinta.
            current_seq.shuffle(&mut rng);

            let makespan = self.first_improvement(
                jsp,
                &mut current_seq,
                max_machine_id,
                stop,
                &mut iter_count,
            );

            if makespan < best_makespan {
                best_makespan = makespan;
                best_sequence = Some(current_seq.clone());
            }
        }

        let best_seq = best_sequence?;
        Some(self.build_schedule(jsp, &best_seq, max_machine_id))
    }
}
