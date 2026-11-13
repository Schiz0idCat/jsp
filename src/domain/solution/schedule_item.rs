//! Elemento de una programación: una operación con sus tiempos asignados.

use std::fmt::Display;

/// Asignación de una operación específica dentro de una
/// [`Schedule`](super::Schedule).
///
/// Registra qué trabajo y operación se ejecutan, en qué máquina y con qué
/// tiempos de inicio, fin y duración.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScheduleItem {
    /// Identificador del trabajo.
    job: usize,

    /// Número de la operación dentro del trabajo.
    op: usize,

    /// Máquina asignada a la operación.
    machine: usize,

    /// Instante de inicio de la operación.
    start: usize,

    /// Instante de término de la operación.
    end: usize,

    /// Duración de la operación.
    duration: usize,
}

// Inicializadores
impl ScheduleItem {
    /// Crea un elemento de programación con todos sus datos.
    pub fn new(
        job: usize,
        op: usize,
        machine: usize,
        start: usize,
        end: usize,
        duration: usize,
    ) -> Self {
        Self {
            job,
            op,
            machine,
            start,
            end,
            duration,
        }
    }
}

// Getters
impl ScheduleItem {
    /// Retorna el identificador del trabajo.
    pub fn job(self) -> usize {
        self.job
    }

    /// Retorna el número de la operación dentro del trabajo.
    pub fn op(self) -> usize {
        self.op
    }
}

impl Display for ScheduleItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<6}{:<6}{:<10}{:<8}{:<8}{:<10}",
            self.job, self.op, self.machine, self.start, self.end, self.duration
        )
    }
}
