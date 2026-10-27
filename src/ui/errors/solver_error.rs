use crate::domain::errors::JspFileError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SolverError {
    #[error("Failed to read JSP file: {0}")]
    File(#[from] JspFileError),

    #[error("Failed to set Ctrl+C handler: {0}")]
    CtrlC(#[from] ctrlc::Error),
}
