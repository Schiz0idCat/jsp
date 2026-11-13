//! Elemento de una programación: una operación con sus tiempos asignados.
//!
//! Corresponde a una fila de la tabla de programación. En el modelo:
//! - `job` identifica el trabajo `j`,
//! - `op` identifica la operación `r` dentro de `O_j`,
//! - `machine` es la máquina `o_jr`,
//! - `start` es el tiempo de inicio `x_ij`,
//! - `duration` es el tiempo de procesamiento `p`,
//! - `end` es el instante de término `x_ij + p`, que no es una variable
//!   independiente pero se guarda para facilitar la lectura.

use std::fmt::Display;

/// Asignación de una operación específica dentro de una
/// [`Schedule`](super::Schedule).
///
/// Registra qué trabajo y operación se ejecutan, en qué máquina y con qué
/// tiempos de inicio, fin y duración.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScheduleItem {
    /// Identificador visible del trabajo `j`.
    ///
    /// Es el `id` 1-based de [`Job`](crate::domain::Job), no el índice
    /// 0-based que usan las secuencias de búsqueda.
    job: usize,

    /// Número de la operación `r` dentro del trabajo, comenzando en 1.
    op: usize,

    /// Máquina `o_jr` que procesa la operación.
    ///
    /// Internamente se usa el índice 0-based de la máquina.
    machine: usize,

    /// Instante de inicio `x_ij` de la operación.
    start: usize,

    /// Instante de término de la operación (`start + duration`).
    end: usize,

    /// Tiempo de procesamiento `p` de la operación.
    duration: usize,
}

// Inicializadores
impl ScheduleItem {
    /// Crea un elemento de programación con todos sus datos.
    ///
    /// `start` y `end` deben cumplir `end = start + duration`. La coherencia
    /// con la precedencia y con la capacidad de las máquinas la garantiza el
    /// decodificador que construye la programación.
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
    /// Retorna el identificador visible del trabajo `j`.
    pub fn job(self) -> usize {
        self.job
    }

    /// Retorna el número de la operación `r` dentro del trabajo.
    pub fn op(self) -> usize {
        self.op
    }
}

impl Display for ScheduleItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Columnas: trabajo, operación, máquina, inicio, término y duración.
        write!(
            f,
            "{:<6}{:<6}{:<10}{:<8}{:<8}{:<10}",
            self.job, self.op, self.machine, self.start, self.end, self.duration
        )
    }
}
