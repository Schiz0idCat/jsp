//! Búsqueda local por primera mejora sobre secuencias de trabajos.
//!
//! # Representación y vecindario
//!
//! Una solución se codifica como una secuencia con una aparición por cada
//! operación de cada trabajo. La posición de cada identificador indica el
//! orden en que el decodificador interpreta las operaciones.
//!
//! El vecindario se genera intercambiando dos posiciones de la secuencia. Si
//! las posiciones contienen trabajos distintos, el orden de las operaciones
//! cambia y, por lo tanto, puede cambiar el makespan `C`. Si contienen el
//! mismo trabajo, la secuencia decodificada es equivalente y el movimiento se
//! descarta.
//!
//! La estrategia aplica **primera mejora**: recorre el vecindario y acepta el
//! primer vecino que reduce `C`. Luego reinicia la exploración desde esa nueva
//! solución. Se detiene cuando ningún vecino mejora, es decir, en un mínimo
//! local.

use rand::seq::SliceRandom;

use super::super::SolutionBuilder;
use super::super::Solver;
use crate::domain::Jsp;
use crate::domain::solution::Schedule;
use crate::solver::RandomSeed;
use crate::solver::stop::SearchControl;

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
    /// # Parámetros
    ///
    /// - `jsp`: instancia con los conjuntos `J` y `M` y las operaciones.
    /// - `sequence`: solución codificada actual. Se modifica in-place con los
    ///   intercambios que mejoran el makespan.
    /// - `max_machine_id`: mayor índice interno de máquina, usado para
    ///   dimensionar los arreglos de disponibilidad.
    /// - `control`: control de parada y presupuesto de iteraciones.
    ///
    /// Retorna el makespan `C` de la secuencia al alcanzar el mínimo local o
    /// al solicitarse la detención.
    fn first_improvement(
        &self,
        jsp: &Jsp,
        sequence: &mut [usize],
        max_machine_id: usize,
        control: &mut SearchControl,
    ) -> usize {
        // Valor de `C` de la solución actual.
        let mut current_makespan = self.evaluate_makespan(jsp, sequence, max_machine_id);

        if sequence.len() <= 1 {
            return current_makespan;
        }

        'search: loop {
            // `improved` indica si en esta pasada se encontró un vecino mejor.
            let mut improved = false;

            // Recorre todos los pares de posiciones (i, j) con i < j.
            for i in 0..sequence.len() {
                for j in (i + 1)..sequence.len() {
                    if sequence[i] == sequence[j] {
                        // Intercambiar dos apariciones del mismo trabajo no
                        // cambia la secuencia decodificada: no es un vecino.
                        continue;
                    }

                    if control.tick() {
                        break 'search;
                    }

                    // Genera el vecino en el mismo buffer y lo evalúa.
                    sequence.swap(i, j);
                    let makespan = self.evaluate_makespan(jsp, sequence, max_machine_id);

                    // Primera mejora: se acepta el primer vecino que reduzca C.
                    if makespan < current_makespan {
                        current_makespan = makespan;
                        improved = true;
                        break;
                    } else {
                        // Si no mejora, se revierte para volver a la solución
                        // actual y seguir explorando otros pares.
                        sequence.swap(i, j);
                    }
                }

                if improved {
                    // Se reinicia la exploración desde la nueva solución.
                    break;
                }
            }

            if !improved {
                // Ningún vecino mejora: se alcanzó un mínimo local.
                break;
            }
        }

        current_makespan
    }
}

impl SolutionBuilder for LocalSearch {}

impl Solver for LocalSearch {
    /// Explora varias soluciones iniciales y retorna la mejor encontrada.
    fn solve_with_stop(&self, jsp: &Jsp, control: &mut SearchControl) -> Option<Schedule> {
        // Sin trabajos no hay operaciones que programar.
        if jsp.jobs().is_empty() {
            return None;
        }

        // Mayor índice interno de máquina (0-based), usado para dimensionar
        // los arreglos de disponibilidad.
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

        // `best_makespan` es el menor `C` entre todas las muestras.
        // `best_sequence` guarda la codificación que lo produjo.
        // `current_seq` es la solución que se está mejorando en cada muestra.
        let mut best_makespan = usize::MAX;
        let mut best_sequence: Option<Vec<usize>> = None;
        let mut rng = self.seed.rng();
        let mut current_seq = base_sequence;

        control.start();

        for _ in 0..self.samples {
            if control.should_stop() {
                break;
            }

            // Cada muestra parte de una permutación inicial distinta.
            current_seq.shuffle(&mut rng);

            // Desciende hasta un mínimo local y obtiene su makespan.
            let makespan = self.first_improvement(jsp, &mut current_seq, max_machine_id, control);

            // Conserva la muestra si mejora el mejor resultado global.
            if makespan < best_makespan {
                best_makespan = makespan;
                best_sequence = Some(current_seq.clone());
            }
        }

        // Decodifica la mejor secuencia en la tabla de tiempos de inicio/fin.
        let best_seq = best_sequence?;
        Some(self.build_schedule(jsp, &best_seq, max_machine_id))
    }
}
