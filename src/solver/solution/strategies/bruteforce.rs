//! Búsqueda exhaustiva sobre secuencias de trabajos.
//!
//! # Representación de una solución
//!
//! Una solución se codifica como una secuencia de identificadores internos de
//! trabajo. Si el trabajo `j` tiene `|O_j|` operaciones, su identificador
//! aparece exactamente `|O_j|` veces. Por ejemplo, si el trabajo 0 tiene tres
//! operaciones, la secuencia base contiene `[0, 0, 0]`.
//!
//! Al recorrer la secuencia de izquierda a derecha, la primera aparición del
//! trabajo `j` representa su operación 1, la segunda representa su operación 2,
//! y así sucesivamente. De esta forma, la precedencia `o_j1 -> o_j2 -> ...`
//! siempre se respeta y toda permutación del multiconjunto es una solución
//! factible.
//!
//! La fuerza bruta enumera todas las permutaciones distintas de ese
//! multiconjunto y conserva la que produce el menor makespan `C`.

use super::super::SolutionBuilder;
use super::super::Solver;
use crate::domain::Jsp;
use crate::domain::solution::Schedule;
use crate::solver::stop::SearchControl;

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
    /// La búsqueda se detiene si el [`SearchControl`] lo solicita.
    fn solve_with_stop(&self, jsp: &Jsp, control: &mut SearchControl) -> Option<Schedule> {
        // Sin trabajos no hay operaciones que programar.
        if jsp.jobs().is_empty() {
            return None;
        }

        // `max_machine_id` es el mayor índice interno de máquina (0-based).
        // Se usa para dimensionar los arreglos de disponibilidad
        // de máquinas al evaluar una secuencia.
        let max_machine_id = jsp
            .jobs()
            .iter()
            .flat_map(|job| job.operations().iter())
            .map(|op| op.machine())
            .max()?;

        // Secuencia base del multiconjunto de operaciones. Cada aparición del
        // índice `idx` representa la siguiente operación pendiente del trabajo
        // `idx`. Por ejemplo, el trabajo 0 con tres operaciones aporta
        // `[0, 0, 0]`.
        let mut base_sequence = Vec::new();
        for (idx, job) in jsp.jobs().iter().enumerate() {
            for _ in 0..job.operations().len() {
                base_sequence.push(idx);
            }
        }

        // `best_makespan` es el menor valor de `C` encontrado hasta ahora.
        // `best_sequence` guarda la codificación que produjo ese valor.
        // `current_seq` es la permutación que se evalúa en la iteración actual.
        let mut best_makespan = usize::MAX;
        let mut best_sequence: Option<Vec<usize>> = None;
        let mut current_seq = base_sequence;

        control.start();

        if !current_seq.is_empty() {
            loop {
                if control.tick() {
                    break;
                }

                // Decodifica la secuencia y obtiene el makespan `C`.
                let makespan = self.evaluate_makespan(jsp, &current_seq, max_machine_id);

                // Se conserva la secuencia si mejora el menor makespan visto.
                if makespan < best_makespan {
                    best_makespan = makespan;
                    best_sequence = Some(current_seq.clone());
                }

                // Avanza a la siguiente permutación lexicográfica.
                // Cuando no quedan permutaciones, termina.
                if !next_permutation(&mut current_seq) {
                    break;
                }
            }
        }

        // `best_sequence` es la mejor codificación encontrada. Se decodifica
        // una última vez para construir la tabla de tiempos de inicio y fin.
        let best_seq = best_sequence?;
        Some(self.build_schedule(jsp, &best_seq, max_machine_id))
    }
}

/// Genera la siguiente permutación lexicográfica de `arr`.
///
/// Como `arr` puede contener identificadores repetidos (un trabajo aparece una
/// vez por cada operación), esta función recorre solo permutaciones distintas
/// del conjunto, no todas las permutaciones de posiciones.
///
/// Retorna `true` si existe una permutación siguiente y `false` si `arr` ya
/// está ordenado de mayor a menor.
fn next_permutation<T: Ord>(arr: &mut [T]) -> bool {
    if arr.len() <= 1 {
        return false; // No hay nada que permutar.
    }

    // Desde la derecha, busca el primer par que no sea descendente. Ese par
    // marca el punto donde aún se puede generar una permutación mayor.
    let mut i = arr.len() - 1;
    while i > 0 && arr[i - 1] >= arr[i] {
        i -= 1;
    }

    if i == 0 {
        return false; // La lista ya está completamente invertida.
    }

    let idx_pivot = i - 1;
    let pivot = &arr[idx_pivot]; // Primer elemento del par no descendente.

    // Desde la derecha, busca el menor elemento mayor que el pivote para
    // intercambiarlo y luego ordenar el sufijo de menor a mayor.
    let mut j = arr.len() - 1;
    while arr[j] <= *pivot {
        j -= 1;
    }

    arr.swap(idx_pivot, j);
    arr[i..].reverse();

    true
}
