//! Generación de un vecino mediante un intercambio de posiciones.
//!
//! Esta estrategia no es una búsqueda completa: construye una solución
//! inicial aleatoria, genera **un** vecino con un `swap` y devuelve ese
//! vecino. Sirve para ilustrar cómo un movimiento cambia la secuencia de
//! trabajos y, con ella, el makespan `C`.
//!
//! Un `swap` intercambia dos posiciones de la secuencia. Si las posiciones
//! contienen el mismo trabajo, la secuencia decodificada no cambia. Por eso
//! solo se consideran pares de posiciones con trabajos distintos.

use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;

use crate::domain::{Jsp, solution::Schedule};
use crate::solver::RandomSeed;
use crate::solver::solution::{Solver, solution_builder::SolutionBuilder};
use crate::solver::stop::StopHandle;

/// Estrategia que genera soluciones vecinas mediante intercambios.
pub struct Swap {
    /// Semilla del generador aleatorio.
    ///
    /// Controla tanto la solución inicial como la elección del par de
    /// posiciones que se intercambia.
    seed: RandomSeed,
}

impl Swap {
    /// Crea una nueva instancia de la estrategia.
    pub fn new(seed: RandomSeed) -> Self {
        Self { seed }
    }

    /// Intercambia dos posiciones sin modificar la solución original.
    ///
    /// `solucion` es la secuencia codificada de trabajos. `i` y `j` son
    /// posiciones dentro de esa secuencia. Retorna una copia con ambos
    /// elementos intercambiados, de modo que la solución original permanece
    /// intacta.
    pub fn swap(&self, solucion: &[usize], i: usize, j: usize) -> Vec<usize> {
        let mut vecino = solucion.to_vec();
        vecino.swap(i, j);
        vecino
    }
}

// Permite utilizar las funciones de evaluación del proyecto.
impl SolutionBuilder for Swap {}

// Implementa la estrategia Swap.
impl Solver for Swap {
    /// Genera una solución aleatoria, aplica un intercambio y retorna el vecino.
    fn solve_with_stop(&self, jsp: &Jsp, stop: &mut StopHandle) -> Option<Schedule> {
        // Sin trabajos no hay operaciones que programar.
        if jsp.jobs().is_empty() {
            return None;
        }

        // Mayor índice interno de máquina (0-based), usado para dimensionar
        // los arreglos de disponibilidad durante la evaluación.
        let max_machine_id = jsp
            .jobs()
            .iter()
            .flat_map(|job| job.operations().iter())
            .map(|op| op.machine())
            .max()?;

        // Secuencia base: cada trabajo se repite una vez por operación.
        let mut solution = Vec::new();
        for (idx, job) in jsp.jobs().iter().enumerate() {
            for _ in 0..job.operations().len() {
                solution.push(idx);
            }
        }

        // Se necesitan al menos dos posiciones para poder intercambiar.
        if solution.len() < 2 {
            return None;
        }

        // Mezcla los trabajos para generar una solución inicial aleatoria.
        let mut rng = self.seed.rng();
        solution.shuffle(&mut rng);

        // Inicia el control de detención.
        stop.start();

        // Detiene la ejecución si se solicita antes de evaluar.
        if stop.should_stop() {
            return None;
        }

        // `makespan` es el valor de `C` de la solución inicial.
        let makespan = self.evaluate_makespan(jsp, &solution, max_machine_id);

        // Guarda los pares de posiciones que contienen trabajos distintos.
        // Un par con el mismo trabajo produciría un vecino idéntico, porque
        // las apariciones del trabajo son indistinguibles para el decodificador.
        let mut valid_pairs = Vec::new();

        for i in 0..solution.len() {
            for j in (i + 1)..solution.len() {
                // Evita intercambiar trabajos iguales.
                if solution[i] != solution[j] {
                    valid_pairs.push((i, j));
                }
            }
        }

        // Selecciona un par de posiciones al azar entre los movimientos que
        // sí cambian la secuencia de trabajos.
        let &(i, j) = valid_pairs.choose(&mut rng)?;

        // Genera la secuencia vecina intercambiando las posiciones.
        let vecino = self.swap(&solution, i, j);

        // Calcula el makespan del vecino para compararlo con el inicial.
        let makespan_vecino = self.evaluate_makespan(jsp, &vecino, max_machine_id);

        // Muestra ambos valores. Esta estrategia es demostrativa y no
        // conserva el mejor de los dos.
        println!("Makespan inicial: {}", makespan);
        println!("Makespan vecino: {}", makespan_vecino);

        // Decodifica el vecino en una programación completa.
        Some(self.build_schedule(jsp, &vecino, max_machine_id))
    }
}
