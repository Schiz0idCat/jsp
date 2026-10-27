use super::super::SolutionBuilder;
use super::super::Solver;
use crate::domain::Jsp;
use crate::domain::solution::Schedule;
use crate::solver::stop::StopHandle;

/// Estrategia de [`Jsp`] resuelto por fuerza bruta.
/// Evalúa exhaustiavemente todas las permutaciones válidas,
/// hasta encontrar la que tenga el mínimo makespan.
pub struct BruteForce;

impl BruteForce {
    /// Inizializador lógico.
    pub fn new() -> Self {
        Self
    }
}

impl SolutionBuilder for BruteForce {}

impl Solver for BruteForce {
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
                base_sequence.push(idx); // si job 0 tiene 3 ops -> [0, 0, 0]
            }
        }

        let mut best_makespan = usize::MAX;
        let mut best_sequence: Option<Vec<usize>> = None;
        let mut current_seq = base_sequence;

        stop.start();
        const CHECK_INTERVAL: u64 = 100_000;
        let mut iter_count: u64 = 0;

        if !current_seq.is_empty() {
            loop {
                if iter_count % CHECK_INTERVAL == 0 && stop.should_stop() {
                    break;
                }
                iter_count += 1;

                let makespan = self.evaluate_makespan(jsp, &current_seq, max_machine_id);

                if makespan < best_makespan {
                    best_makespan = makespan;
                    best_sequence = Some(current_seq.clone());
                }

                if !next_permutation(&mut current_seq) {
                    break;
                }
            }
        }

        let best_seq = best_sequence?;
        Some(self.build_schedule(jsp, &best_seq, max_machine_id))
    }
}

fn next_permutation<T: Ord>(arr: &mut [T]) -> bool {
    if arr.len() <= 1 {
        return false; // no hay que permutar
    }

    // Desde la derecha, busca un par de elementos no sea descendente
    let mut i = arr.len() - 1;
    while i > 0 && arr[i - 1] >= arr[i] {
        i -= 1;
    }

    if i == 0 {
        return false; // si no encuentra ningún par, la lista ya está completamente invertida
    }

    let idx_pivot = i - 1;
    let pivot = &arr[idx_pivot]; // primer elemento del par no descendente

    // Desde la derecha, busca el elemento más pequeño que sea mayor al pivote
    let mut j = arr.len() - 1;
    while arr[j] <= *pivot {
        j -= 1;
    }

    arr.swap(idx_pivot, j);
    arr[i..].reverse();

    true
}
