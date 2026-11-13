//! Error producido al preparar la ejecución del solucionador.

use crate::domain::errors::JspFileError;
use thiserror::Error;

/// Errores que puede producir la preparación de [`SolverData`](crate::ui::SolverData).
#[derive(Debug, Error)]
pub enum SolverError {
    /// Error al leer o parsear el archivo de la instancia.
    #[error("Failed to read JSP file: {0}")]
    File(#[from] JspFileError),

    /// Error al registrar el manejador de `Ctrl+C`.
    #[error("Failed to set Ctrl+C handler: {0}")]
    CtrlC(#[from] ctrlc::Error),
}
