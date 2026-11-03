use thiserror::Error;

/// Error de parseo.
#[derive(Debug, Error)]
pub enum JspParseError {
    /// El archivo está vacio.
    #[error("The content to parse is empty")]
    EmptyInput,

    /// Header inválido.
    #[error("Header must contain two integers (n_jobs n_machines)")]
    InvalidHeader,

    /// Error al parsear un entero.
    #[error("Failed to parse an integer in the input")]
    InvalidNumber(#[from] std::num::ParseIntError),

    /// Error de formato en la especificación de las máquinas.
    #[error("Job line contains an odd number of values (expected machine duration pairs)")]
    OddOperationValues,
}
