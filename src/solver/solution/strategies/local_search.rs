//! Esta estrategia resuelve el taller 7

use rand::rng;
use rand::seq::SliceRandom;

use super::super::SolutionBuilder;
use super::super::Solver;
use crate::domain::Jsp;
use crate::domain::solution::Schedule;
use crate::solver::stop::StopHandle;

/// Estrategia de [`Jsp`] resuelto mediante búsqueda local por primera mejora.
/// Genera soluciones iniciales aleatorias y explora iterativamente su vecindario
/// mediante intercambios (swap) hasta alcanzar un óptimo local.
pub struct LocalSearch {
    samples: usize,
}

impl LocalSearch {
    /// Inicializador lógico.
    pub fn new(samples: usize) -> Self {
        Self { samples }
    }

    /// Aplica búsqueda local completa con criterio de aceptación por primera mejora.
    /// Explora el vecindario de la secuencia mediante el operador swap hasta alcanzar
    /// un óptimo local o hasta que se cumpla una condición de parada.
    fn first_improvement(
        &self,
        jsp: &Jsp,
        sequence: &mut [usize],
        max_machine_id: usize,
        stop: &StopHandle,
        iter_count: &mut u64,
    ) -> usize {
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
                        continue; // intercambiar el mismo trabajo no altera la secuencia
                    }

                    if *iter_count % CHECK_INTERVAL == 0 && stop.should_stop() {
                        break 'search;
                    }
                    *iter_count += 1;

                    sequence.swap(i, j);
                    let makespan = self.evaluate_makespan(jsp, sequence, max_machine_id);

                    // Criterio de primera mejora: se acepta el primer vecino que mejore la solución
                    if makespan < current_makespan {
                        current_makespan = makespan;
                        improved = true;
                        break;
                    } else {
                        sequence.swap(i, j); // revierte el movimiento si no mejora
                    }
                }

                if improved {
                    break; // reinicia la exploración desde la nueva solución actual
                }
            }

            if !improved {
                break; // se alcanzó un óptimo local (ningún vecino mejora la solución)
            }
        }

        current_makespan
    }
}

impl SolutionBuilder for LocalSearch {}

impl Solver for LocalSearch {
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
        let mut iter_count: u64 = 0;

        for _ in 0..self.samples {
            if stop.should_stop() {
                break;
            }

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
