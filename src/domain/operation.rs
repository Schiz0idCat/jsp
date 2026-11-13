//! Representa una operación individual de un trabajo.

use std::fmt::Display;

/// Operación individual dentro de un [`Job`](super::Job).
///
/// Una operación indica la máquina en la que debe ejecutarse y su duración.
pub struct Operation {
    /// Identificador de la máquina que procesa la operación.
    machine: usize,

    /// Duración de la operación en unidades de tiempo.
    duration: usize,
}

// Inicializadores
impl Operation {
    /// Crea una operación para una máquina y una duración dadas.
    pub fn new(machine: usize, duration: usize) -> Self {
        Self { machine, duration }
    }
}

// Getters
impl Operation {
    /// Retorna la máquina en la que se ejecuta la operación.
    pub fn machine(&self) -> usize {
        self.machine
    }

    /// Retorna la duración de la operación.
    pub fn duration(&self) -> usize {
        self.duration
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "M{} ({}h)", self.machine, self.duration)
    }
}
