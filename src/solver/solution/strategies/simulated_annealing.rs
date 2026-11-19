//! Simulated Annealing para el Job Shop Problem.
//!
//! # Representación de la solución
//!
//! La solución se codifica como una secuencia con una aparición por cada
//! operación de cada trabajo. Al decodificarla de izquierda a derecha, la
//! primera aparición de un trabajo corresponde a su operación 1, la segunda
//! a su operación 2, y así sucesivamente. Así se respeta la precedencia
//! `o_j1 -> o_j2 -> ...` y la solución siempre es factible respecto de las
//! restricciones del JSP.
//!
//! # Función objetivo
//!
//! El valor de una solución es su makespan `C`, el instante en que termina la
//! última operación. El objetivo es minimizar `C`.
//!
//! # Aceptación de soluciones peores
//!
//! A diferencia de la búsqueda local, Simulated Annealing puede aceptar un
//! vecino con mayor makespan. La probabilidad de aceptarlo es
//! `exp(-delta / T)`, donde `delta = C(vecino) - C(actual)` y `T` es la
//! temperatura actual. Al comenzar, `T` es alta y permite explorar. Al
//! enfriarse, la probabilidad de aceptar empeoramientos disminuye.

use crate::domain::{Jsp, solution::Schedule};
use crate::solver::RandomSeed;
use crate::solver::solution::{Solver, solution_builder::SolutionBuilder};
use crate::solver::stop::SearchControl;

use rand::RngExt;
use rand::seq::SliceRandom;

/// Estrategia de solución basada en Simulated Annealing.
pub struct SimulatedAnnealing {
    /// Temperatura inicial `T_0`.
    ///
    /// Controla la probabilidad de aceptar soluciones peores al comienzo. Una
    /// temperatura alta permite explorar soluciones alejadas de la actual.
    t_initial: f64,

    /// Temperatura mínima `T_min`.
    ///
    /// Cuando `T` baja de este valor, el ciclo termina.
    t_min: f64,

    /// Tasa de enfriamiento `alpha`.
    ///
    /// En cada iteración se actualiza `T <- alpha * T`. Debe cumplir
    /// `0 < alpha < 1`. Mientras más cerca de 1, más lento es el enfriamiento.
    alpha: f64,

    /// Semilla del generador aleatorio.
    ///
    /// Controla la solución inicial, la elección de los vecinos y la
    /// aceptación probabilística.
    seed: RandomSeed,
}

impl SimulatedAnnealing {
    /// Crea la estrategia con sus parámetros de temperatura y su semilla.
    pub fn new(t_initial: f64, t_min: f64, alpha: f64, seed: RandomSeed) -> Self {
        Self {
            t_initial,
            t_min,
            alpha,
            seed,
        }
    }

    /// Ejecuta el ciclo de enfriamiento notificando el mejor makespan por iteración.
    ///
    /// El callback recibe `(iteración, mejor_makespan)`, donde la iteración
    /// comienza en `1`. El makespan notificado es el mejor encontrado hasta
    /// esa iteración, no el de la solución actual.
    pub fn solve_with_trace<F>(
        &self,
        jsp: &Jsp,
        control: &mut SearchControl,
        mut on_iteration: F,
    ) -> Option<Schedule>
    where
        F: FnMut(u64, usize),
    {
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

        // Secuencia inicial: una aparición por cada operación de cada trabajo.
        // Luego se mezcla para partir de una solución aleatoria factible.
        let mut current_seq: Vec<usize> = jsp
            .jobs()
            .iter()
            .enumerate()
            .flat_map(|(idx, job)| std::iter::repeat_n(idx, job.operations().len()))
            .collect();

        let mut rng = self.seed.rng();
        current_seq.shuffle(&mut rng);

        // `current_makespan` es el valor de `C` de la solución actual.
        let mut current_makespan = self.evaluate_makespan(jsp, &current_seq, max_machine_id);

        // `best_makespan` guarda el menor `C` visto, aunque la solución actual
        // sea peor. `best_seq` es la codificación que produjo ese valor.
        let mut best_seq = current_seq.clone();
        let mut best_makespan = current_makespan;

        // Temperatura actual. Comienza en `T_0` y se multiplica por `alpha`
        // en cada iteración.
        let mut t = self.t_initial;
        control.start();

        while t > self.t_min {
            if control.tick() {
                break;
            }

            // Selecciona dos posiciones distintas de la secuencia. Deben
            // contener trabajos distintos. Si no, el intercambio no cambia la
            // solución decodificada.
            let i = rng.random_range(0..current_seq.len());
            let mut j = rng.random_range(0..current_seq.len());

            // Se sortea `j` hasta que sea distinto de `i` y contenga otro
            // trabajo. Si `i == j`, el swap no modifica la secuencia. Si ambas
            // posiciones contienen el mismo trabajo, la secuencia de
            // identificadores tampoco cambia, porque las apariciones de un
            // trabajo son indistinguibles para el decodificador. Por eso se
            // necesita un par de posiciones con trabajos distintos. El bucle
            // supone que existe al menos un par válido.
            while i == j || current_seq[i] == current_seq[j] {
                j = rng.random_range(0..current_seq.len());
            }

            // Genera el vecino intercambiando ambas posiciones.
            current_seq.swap(i, j);
            let neighbor_makespan = self.evaluate_makespan(jsp, &current_seq, max_machine_id);

            // `delta` mide cuánto empeora (positivo) o mejora (negativo) el
            // makespan del vecino respecto de la solución actual.
            let delta = neighbor_makespan as f64 - current_makespan as f64;

            // Criterio de aceptación de Metropolis:
            // - si `delta < 0`, el vecino mejora y se acepta siempre,
            // - si `delta >= 0`, se acepta con probabilidad `exp(-delta / T)`.
            if delta < 0.0 || rng.random::<f64>() < (-delta / t).exp() {
                // Se acepta el movimiento.
                current_makespan = neighbor_makespan;

                // Si además es el mejor visto hasta ahora, se actualiza el
                // registro global de la mejor solución.
                if current_makespan < best_makespan {
                    best_makespan = current_makespan;
                    best_seq = current_seq.clone();
                }
            } else {
                // Se rechaza el movimiento y se revierte el intercambio.
                current_seq.swap(i, j);
            }

            // Notifica el mejor makespan acumulado después de esta iteración.
            on_iteration(control.iterations(), best_makespan);

            // Enfriamiento geométrico: la temperatura baja gradualmente.
            t *= self.alpha;
        }

        // Decodifica la mejor secuencia encontrada en la tabla de tiempos de
        // inicio y fin de cada operación.
        Some(self.build_schedule(jsp, &best_seq, max_machine_id))
    }
}

impl SolutionBuilder for SimulatedAnnealing {}

impl Solver for SimulatedAnnealing {
    /// Ejecuta el ciclo de enfriamiento y retorna la mejor solución encontrada.
    fn solve_with_stop(&self, jsp: &Jsp, control: &mut SearchControl) -> Option<Schedule> {
        self.solve_with_trace(jsp, control, |_, _| {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Job, Operation};

    #[test]
    fn trace_records_each_iteration_in_order() {
        let jsp = Jsp::new(
            2,
            1,
            vec![
                Job::new(1, vec![Operation::new(0, 5)]),
                Job::new(2, vec![Operation::new(0, 5)]),
            ],
        );
        let solver = SimulatedAnnealing::new(10.0, 1.0, 0.5, RandomSeed::from_u64(1));
        let mut control = SearchControl::default();
        let mut trace = Vec::new();

        let schedule = solver
            .solve_with_trace(&jsp, &mut control, |iteration, makespan| {
                trace.push((iteration, makespan));
            })
            .expect("debe existir una solución");

        assert!(!trace.is_empty());
        assert_eq!(
            trace
                .iter()
                .map(|(iteration, _)| *iteration)
                .collect::<Vec<_>>(),
            (1..=trace.len() as u64).collect::<Vec<_>>()
        );
        assert_eq!(trace.last().unwrap().1, 10);
        assert!(format!("{schedule}").contains("Best Makespan: 10"));
    }
}
