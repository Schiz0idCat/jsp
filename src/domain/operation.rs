//! Operación individual de un trabajo.
//!
//! En el modelo, la operación `r` del trabajo `j` se denota `o_jr`.
//! Cada operación tiene dos datos:
//! - la máquina `o_jr ∈ M` que debe procesarla.
//! - su tiempo de procesamiento `p`.
//!
//! La operación no puede interrumpirse una vez iniciada y no puede comenzar
//! antes de que termine la operación anterior del mismo trabajo.

use std::fmt::Display;

/// Operación `o_jr` dentro de un [`Job`](super::Job).
///
/// Una operación indica la máquina que la procesa y su duración. El trabajo
/// al que pertenece y su número de operación se conocen por la posición que
/// ocupa dentro de `Job::operations`.
pub struct Operation {
    /// Máquina `o_jr` que procesa la operación.
    ///
    /// Internamente es un índice 0-based (`0..n_machines`), aunque el modelo y
    /// la salida usan la numeración 1-based (`M1..Mm`). Este índice se usa
    /// para acceder a los arreglos de disponibilidad de máquinas.
    machine: usize,

    /// Tiempo de procesamiento `p` de la operación.
    ///
    /// Se expresa en las mismas unidades de tiempo que el makespan y que los
    /// tiempos de inicio y término de la programación.
    duration: usize,
}

// Inicializadores
impl Operation {
    /// Crea una operación con su máquina y su duración.
    pub fn new(machine: usize, duration: usize) -> Self {
        Self { machine, duration }
    }
}

// Getters
impl Operation {
    /// Retorna la máquina `o_jr` que procesa la operación.
    pub fn machine(&self) -> usize {
        self.machine
    }

    /// Retorna el tiempo de procesamiento `p` de la operación.
    pub fn duration(&self) -> usize {
        self.duration
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Se imprime el índice interno de la máquina tal como lo usa el programa.
        // El modelo matemático la numera desde 1, sin embargo,
        // el código la indexa desde 0 para acceder a sus arreglos.
        write!(f, "M{} ({}h)", self.machine, self.duration)
    }
}
