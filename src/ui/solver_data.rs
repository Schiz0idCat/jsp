//! Encapsula la instancia, la estrategia y el control de parada.

use super::Strategy;
use super::solver_factory;
use crate::domain::Jsp;
use crate::domain::solution::Schedule;
use crate::solver::RandomSeed;
use crate::solver::solution::Solver;
use crate::solver::stop::{
    CancelCondition, SearchBudget, SearchControl, StopHandle, TimeoutCondition,
};
use crate::ui::errors::SolverError;

use std::num::NonZeroU64;
use std::path::Path;
use std::time::Duration;

/// Encapsula una instancia [`Jsp`], su control de parada y la estrategia
/// seleccionada para resolverla.
pub struct SolverData {
    /// Instancia cargada desde el archivo.
    jsp: Jsp,

    /// Estrategia de solución seleccionada.
    solver: Box<dyn Solver>,

    /// Condiciones externas y presupuesto de iteraciones.
    control: SearchControl,

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
        max_iterations: Option<NonZeroU64>,
        strategy: Strategy,
        seed: RandomSeed,
    ) -> Result<Self, SolverError> {
        let jsp = Jsp::from_file(filepath)?;

        let cancel = CancelCondition::handle_ctrlc()?;

        // La cancelación siempre está activa. El tiempo y las iteraciones son
        // límites opcionales.
        let mut stop = StopHandle::new().with_condition(cancel.clone());

        if let Some(time) = timeout {
            stop = stop.with_condition(TimeoutCondition::new(time));
        }

        let control = SearchControl::new(stop, SearchBudget::new(max_iterations));
        let solver = solver_factory::build(strategy, seed);

        Ok(Self {
            jsp,
            solver,
            control,
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
        self.solver.solve_with_stop(&self.jsp, &mut self.control)
    }
}
