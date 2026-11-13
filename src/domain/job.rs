//! Trabajo del Job Shop Problem.
//!
//! En el modelo del enunciado, el conjunto de trabajos se denota por `J = {1, 2, ..., n}`.
//! Cada trabajo `j` tiene una secuencia ordenada de operaciones `O_j = {o_j1, o_j2, ..., o_jm}`.
//! Esa secuencia no se puede alterar:
//! la operación `r + 1` solo puede comenzar cuando terminó la operación `r`.

use std::fmt::Display;

use super::Operation;

/// Trabajo `j` de una instancia JSP.
///
/// Contiene el identificador visible del trabajo y su secuencia de
/// operaciones. El orden del vector `operations` representa exactamente la
/// precedencia tecnológica `o_j1 -> o_j2 -> ... -> o_jm`.
pub struct Job {
    /// Identificador visible del trabajo (`j` en el modelo, normalmente 1..n).
    id: usize,

    /// Secuencia de operaciones `O_j`.
    ///
    /// La posición `r` del vector corresponde a la operación `r + 1` del trabajo.
    /// Cada elemento indica en qué máquina se ejecuta esa operación y cuánto demora.
    operations: Vec<Operation>,
}

// Inicializadores
impl Job {
    /// Crea un trabajo con su identificador visible y sus operaciones.
    ///
    /// `id` se usa para la salida del programa. `operations` debe venir en el
    /// orden de precedencia definido por el archivo de la instancia.
    pub fn new(id: usize, operations: Vec<Operation>) -> Self {
        Self { id, operations }
    }
}

// Getters
impl Job {
    /// Retorna la secuencia `O_j` en orden de precedencia.
    pub fn operations(&self) -> &[Operation] {
        &self.operations
    }

    /// Retorna el identificador visible del trabajo.
    pub fn id(&self) -> usize {
        self.id
    }
}

impl Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Cada operación se muestra como una columna de ancho fijo. El orden
        // de izquierda a derecha es el orden de precedencia del trabajo.
        let ops = self
            .operations
            .iter()
            .map(|op| format!("{:^10}", op.to_string()))
            .collect::<Vec<_>>()
            .join(" | ");

        write!(f, "Job {:<2} | {}", self.id, ops)
    }
}
