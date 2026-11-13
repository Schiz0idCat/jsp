//! Encapsula la instancia, la estrategia y las condiciones de parada.

use super::Strategy;
use super::solver_factory;
use crate::domain::Jsp;
use crate::domain::solution::Schedule;
use crate::solver::RandomSeed;
use crate::solver::solution::Solver;
use crate::solver::stop::{CancelCondition, StopHandle, TimeoutCondition};
use crate::ui::errors::SolverError;

use std::path::Path;
use std::time::Duration;

/// Encapsula una instancia [`Jsp`], sus condiciones de parada y la estrategia
/// seleccionada para resolverla.
pub struct SolverData {
    /// Instancia cargada desde el archivo.
    jsp: Jsp,

    /// Estrategia de solución seleccionada.
    solver: Box<dyn Solver>,

    /// Condiciones de parada activas.
    stop: StopHandle,

    /// Condición de cancelación compartida con la barra de progreso.
    cancel: CancelCondition,
}

impl SolverData {
    /// Inicializa y valida los parámetros para resolver el problema.
    ///
    /// # Errors
    ///
    /// Retorna [`SolverError`] si el archivo de la instancia no se puede leer,
    /// no se puede parsear o si falla el registro del manejador de `Ctrl+C`.
    pub fn new(
        filepath: &Path,
        timeout: Option<Duration>,
        strategy: Strategy,
        seed: RandomSeed,
    ) -> Result<Self, SolverError> {
        let jsp = Jsp::from_file(filepath)?;

        let cancel = CancelCondition::handle_ctrlc()?;

        // La cancelación siempre está activa; el tiempo límite es opcional.
        let mut stop = StopHandle::new().with_condition(cancel.clone());

        if let Some(time) = timeout {
            stop = stop.with_condition(TimeoutCondition::new(time));
        }

        let solver = solver_factory::build(strategy, seed);

        Ok(Self {
            jsp,
            solver,
            stop,
            cancel,
        })
    }
}

impl SolverData {
    /// Retorna una referencia a la instancia JSP.
    pub fn jsp(&self) -> &Jsp {
        &self.jsp
    }

    /// Retorna una referencia a la condición de cancelación.
    pub fn cancel_handle(&self) -> &CancelCondition {
        &self.cancel
    }

    /// Ejecuta la estrategia seleccionada y retorna su solución.
    pub fn solve(&mut self) -> Option<Schedule> {
        self.solver.solve_with_stop(&self.jsp, &mut self.stop)
    }
}
