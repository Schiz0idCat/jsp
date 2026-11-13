//! Estrategia de búsqueda exhaustiva por permutaciones.

use super::super::SolutionBuilder;
use super::super::Solver;
use crate::domain::Jsp;
use crate::domain::solution::Schedule;
use crate::solver::stop::StopHandle;

/// Estrategia de [`Jsp`] resuelta por fuerza bruta.
///
/// Evalúa exhaustivamente todas las permutaciones válidas de la secuencia de
/// trabajos y conserva la que tiene el menor makespan encontrado.
pub struct BruteForce;

impl BruteForce {
    /// Crea una instancia de la estrategia.
    pub fn new() -> Self {
        Self
    }
}

impl Default for BruteForce {
    fn default() -> Self {
        Self::new()
    }
}

impl SolutionBuilder for BruteForce {}

impl Solver for BruteForce {
    /// Recorre todas las permutaciones y retorna la mejor programación.
    ///
    /// La búsqueda se detiene si el [`StopHandle`] lo solicita.
    fn solve_with_stop(&self, jsp: &Jsp, stop: &mut StopHandle) -> Option<Schedule> {
        if jsp.jobs().is_empty() {
            return None;
        }

        // La máquina con el identificador más alto determina el tamaño de los
        // arreglos de disponibilidad usados al evaluar una secuencia.
        let max_machine_id = jsp
            .jobs()
            .iter()
            .flat_map(|job| job.operations().iter())
            .map(|op| op.machine())
            .max()?;

        // Construye la secuencia base con cada trabajo
        // repetido según su cantidad de operaciones.
        // Por ejemplo, el trabajo 0 con tres operaciones aporta [0, 0, 0].
        let mut base_sequence = Vec::new();
        for (idx, job) in jsp.jobs().iter().enumerate() {
            for _ in 0..job.operations().len() {
                base_sequence.push(idx);
            }
        }

        let mut best_makespan = usize::MAX;
        let mut best_sequence: Option<Vec<usize>> = None;
        let mut current_seq = base_sequence;

        stop.start();

        // Se consulta la condición de parada cada cierto número de permutaciones
        // para no penalizar en exceso el recorrido exhaustivo.
        const CHECK_INTERVAL: u64 = 100_000;
        let mut iter_count: u64 = 0;

        if !current_seq.is_empty() {
            loop {
                if iter_count.is_multiple_of(CHECK_INTERVAL) && stop.should_stop() {
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

/// Genera la siguiente permutación lexicográfica de `arr`.
///
/// Retorna `true` si existe una permutación siguiente y `false` si `arr` ya
/// está ordenado de mayor a menor.
fn next_permutation<T: Ord>(arr: &mut [T]) -> bool {
    if arr.len() <= 1 {
        return false; // No hay nada que permutar.
    }

    // Desde la derecha, busca el primer par que no sea descendente.
    let mut i = arr.len() - 1;
    while i > 0 && arr[i - 1] >= arr[i] {
        i -= 1;
    }

    if i == 0 {
        return false; // La lista ya está completamente invertida.
    }

    let idx_pivot = i - 1;
    let pivot = &arr[idx_pivot]; // Primer elemento del par no descendente.

    // Desde la derecha, busca el menor elemento mayor que el pivote.
    let mut j = arr.len() - 1;
    while arr[j] <= *pivot {
        j -= 1;
    }

    arr.swap(idx_pivot, j);
    arr[i..].reverse();

    true
}
