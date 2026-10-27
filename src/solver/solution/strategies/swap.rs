
 //! Esta estrategia resuelve el taller 6

// Librerías para generar números y selecciones aleatorias.
use rand::prelude::IndexedRandom;
use rand::rng;
use rand::seq::SliceRandom;

// Estructuras necesarias para trabajar con el problema.
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
    pub fn intercambiar(
        &self,
        solucion: &[usize],
        i: usize,
        j: usize,
    ) -> Vec<usize> {
        // Copia la solución original.
        let mut vecino = solucion.to_vec();

        // Intercambia las posiciones seleccionadas.
        vecino.swap(i, j);

        // Devuelve la solución vecina.
        vecino
    }
}

// Permite utilizar las funciones de evaluación del proyecto.
impl SolutionBuilder for Swap {}

// Implementa la estrategia Swap.
impl Solver for Swap {
    fn solve_with_stop(
        &self,
        jsp: &Jsp,
        stop: &mut StopHandle,
    ) -> Option<Schedule> {
        // Comprueba que existan trabajos.
        if jsp.jobs().is_empty() {
            return None;
        }

        // Obtiene el identificador de máquina más alto.
        let max_machine_id = jsp
            .jobs()
            .iter()
            .flat_map(|job| job.operations().iter())
            .map(|op| op.machine())
            .max()?;

        // Prepara la secuencia inicial de trabajos.
        let mut solucion = Vec::new();

        // Agrega cada trabajo según su cantidad de operaciones.
        for (idx, job) in jsp.jobs().iter().enumerate() {
            for _ in 0..job.operations().len() {
                solucion.push(idx);
            }
        }

        // Comprueba que existan suficientes operaciones.
        if solucion.len() < 2 {
            return None;
        }

        // Mezcla los trabajos para generar una solución aleatoria.
        let mut rng = rng();
        solucion.shuffle(&mut rng);

        // Inicia el control de detención.
        stop.start();

        // Detiene la ejecución si se solicita.
        if stop.should_stop() {
            return None;
        }

        // Calcula el makespan de la solución inicial.
        let makespan_inicial =
            self.evaluate_makespan(jsp, &solucion, max_machine_id);

        // Guarda los pares de posiciones que contienen trabajos distintos.
        let mut pares_validos = Vec::new();

        // Recorre las posiciones de la solución.
        for i in 0..solucion.len() {
            for j in (i + 1)..solucion.len() {
                // Evita intercambiar trabajos iguales.
                if solucion[i] != solucion[j] {
                    pares_validos.push((i, j));
                }
            }
        }

        // Selecciona un par de posiciones al azar.
        let &(i, j) = pares_validos.choose(&mut rng)?;

        // Genera una solución vecina intercambiando las posiciones.
        let vecino = self.intercambiar(&solucion, i, j);

        // Calcula el makespan de la solución vecina.
        let makespan_vecino =
            self.evaluate_makespan(jsp, &vecino, max_machine_id);

        // Muestra los resultados de ambas soluciones.
        println!("Makespan inicial: {}", makespan_inicial);
        println!("Makespan vecino: {}", makespan_vecino);

        // Devuelve la planificación de la solución vecina.
        Some(self.build_schedule(jsp, &vecino, max_machine_id))
    }
}
