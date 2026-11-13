//! Representa un trabajo y la secuencia ordenada de sus operaciones.

use std::fmt::Display;

use super::Operation;

/// Trabajo de una instancia JSP.
///
/// Cada trabajo tiene un identificador y una secuencia de operaciones que
/// deben ejecutarse en el orden definido por el archivo de la instancia.
pub struct Job {
    /// Identificador del trabajo.
    id: usize,

    /// Operaciones del trabajo en orden de precedencia.
    operations: Vec<Operation>,
}

// Inicializadores
impl Job {
    /// Crea un trabajo con su identificador y sus operaciones.
    pub fn new(id: usize, operations: Vec<Operation>) -> Self {
        Self { id, operations }
    }
}

// Getters
impl Job {
    /// Retorna las operaciones del trabajo en orden de precedencia.
    pub fn operations(&self) -> &[Operation] {
        &self.operations
    }

    /// Retorna el identificador del trabajo.
    pub fn id(&self) -> usize {
        self.id
    }
}

impl Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Formatea cada operación en una columna de ancho fijo.
        let ops = self
            .operations
            .iter()
            .map(|op| format!("{:^10}", op.to_string()))
            .collect::<Vec<_>>()
            .join(" | ");

        write!(f, "Job {:<2} | {}", self.id, ops)
    }
}
