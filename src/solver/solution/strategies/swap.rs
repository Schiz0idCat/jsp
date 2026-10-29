//! Esta estrategia resuelve el taller 6

use rand::prelude::IndexedRandom;
use rand::rng;
use rand::seq::SliceRandom;

use crate::domain::{Jsp, solution::Schedule};
use crate::solver::solution::{Solver, solution_builder::SolutionBuilder};
use crate::solver::stop::StopHandle;

// Estrategia que genera soluciones vecinas mediante intercambios.
pub struct Swap;

impl Swap {
    // Crea una nueva instancia de Swap.
    pub fn new() -> Self {
        Self
    }

    // Intercambia dos posiciones sin modificar la solución original.
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

        // Prepara la secuencia inicial de trabajos.
        let mut solution = Vec::new();

        // Agrega cada trabajo según su cantidad de operaciones.
        for (idx, job) in jsp.jobs().iter().enumerate() {
            for _ in 0..job.operations().len() {
                solution.push(idx);
            }
        }

        // Comprueba que existan suficientes operaciones.
        if solution.len() < 2 {
            return None;
        }

        // Mezcla los trabajos para generar una solución aleatoria.
        let mut rng = rng();
        solution.shuffle(&mut rng);

        // Inicia el control de detención.
        stop.start();

        // Detiene la ejecución si se solicita.
        if stop.should_stop() {
            return None;
        }

        // Calcula el makespan de la solución inicial.
        let makespan = self.evaluate_makespan(jsp, &solution, max_machine_id);

        // Guarda los pares de posiciones que contienen trabajos distintos.
        let mut valid_pairs = Vec::new();

        // Recorre las posiciones de la solución.
        for i in 0..solution.len() {
            for j in (i + 1)..solution.len() {
                // Evita intercambiar trabajos iguales.
                if solution[i] != solution[j] {
                    valid_pairs.push((i, j));
                }
            }
        }

        // Selecciona un par de posiciones al azar.
        let &(i, j) = valid_pairs.choose(&mut rng)?;

        // Genera una solución vecina intercambiando las posiciones.
        let vecino = self.swap(&solution, i, j);

        // Calcula el makespan de la solución vecina.
        let makespan_vecino = self.evaluate_makespan(jsp, &vecino, max_machine_id);

        // Muestra los resultados de ambas soluciones.
        println!("Makespan inicial: {}", makespan);
        println!("Makespan vecino: {}", makespan_vecino);

        // Devuelve la planificación de la solución vecina.
        Some(self.build_schedule(jsp, &vecino, max_machine_id))
    }
}
