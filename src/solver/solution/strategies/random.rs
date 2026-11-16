//! Muestreo aleatorio de secuencias de trabajos.
//!
//! Cada solución se codifica como una secuencia con una aparición por cada
//! operación de cada trabajo. Al mezclar esa secuencia se obtiene una solución
//! factible: el decodificador interpreta las operaciones de cada trabajo en el
//! orden `o_j1, o_j2, ...`, respetando la precedencia.
//!
//! Esta estrategia no aprende de las soluciones anteriores: solo genera
//! `samples` secuencias aleatorias, calcula su makespan `C` y conserva la
//! mejor. Sirve como línea base para comparar con las metaheurísticas.

use rand::seq::SliceRandom;

use super::super::SolutionBuilder;
use super::super::Solver;
use crate::domain::Jsp;
use crate::domain::solution::Schedule;
use crate::solver::RandomSeed;
use crate::solver::stop::SearchControl;

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
    fn solve_with_stop(&self, jsp: &Jsp, control: &mut SearchControl) -> Option<Schedule> {
        // Sin trabajos no hay operaciones que programar.
        if jsp.jobs().is_empty() {
            return None;
        }

        // Mayor índice interno de máquina (0-based). Se usa para dimensionar
        // los arreglos que registran cuándo queda libre cada máquina.
        let max_machine_id = jsp
            .jobs()
            .iter()
            .flat_map(|job| job.operations().iter())
            .map(|op| op.machine())
            .max()?;

        // Secuencia base: cada trabajo se repite una vez por operación.
        // El índice `idx` es la posición 0-based del trabajo en `jsp.jobs()`.
        // No es el identificador visible `Job::id`.
        let mut base_sequence = Vec::new();
        for (idx, job) in jsp.jobs().iter().enumerate() {
            for _ in 0..job.operations().len() {
                base_sequence.push(idx);
            }
        }

        // `best_makespan` es el menor `C` encontrado. `best_sequence` es la
        // codificación que lo produjo. `current_seq` es la muestra actual.
        let mut best_makespan = usize::MAX;
        let mut best_sequence: Option<Vec<usize>> = None;
        let mut rng = self.seed.rng();
        let mut current_seq = base_sequence;

        control.start();

        for _ in 0..self.samples {
            if control.tick() {
                break;
            }

            // Mezcla la secuencia actual. Al cambiar el orden de los
            // identificadores de trabajo cambia también el orden en que el
            // decodificador interpreta las operaciones de cada trabajo.
            current_seq.shuffle(&mut rng);

            // Decodifica la secuencia y calcula el makespan `C`.
            let makespan = self.evaluate_makespan(jsp, &current_seq, max_machine_id);

            // Conserva la muestra si mejora el mejor makespan conocido.
            if makespan < best_makespan {
                best_makespan = makespan;
                best_sequence = Some(current_seq.clone());
            }
        }

        // Convierte la mejor codificación en la tabla de tiempos de inicio y
        // fin de cada operación.
        let best_seq = best_sequence?;
        Some(self.build_schedule(jsp, &best_seq, max_machine_id))
    }
}
