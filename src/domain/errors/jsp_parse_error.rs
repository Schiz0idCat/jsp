//! Errores producidos al parsear el texto de una instancia JSP.

use thiserror::Error;

/// Error de parseo del contenido de una instancia.
#[derive(Debug, Error)]
pub enum JspParseError {
    /// El contenido de entrada está vacío.
    #[error("The content to parse is empty")]
    EmptyInput,

    /// La cabecera no contiene los dos enteros esperados.
    #[error("Header must contain two integers (n_jobs n_machines)")]
    InvalidHeader,

    /// Error al convertir un token a un entero.
    #[error("Failed to parse an integer in the input")]
    InvalidNumber(#[from] std::num::ParseIntError),

    /// Una línea de trabajo no contiene pares máquina-duración.
    #[error("Job line contains an odd number of values (expected machine duration pairs)")]
    OddOperationValues,
}
