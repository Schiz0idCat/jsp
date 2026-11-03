use thiserror::Error;

use super::JspParseError;

/// Error de manejo de archivos.
#[derive(Debug, Error)]
pub enum JspFileError {
    /// Error al leer el archivo.
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),

    /// Error al parsear el archivo.
    #[error("Failed to parse JSP content: {0}")]
    Parse(#[from] JspParseError),
}
