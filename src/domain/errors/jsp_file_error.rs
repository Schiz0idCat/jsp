use thiserror::Error;

use super::JspParseError;

#[derive(Debug, Error)]
pub enum JspFileError {
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse JSP content: {0}")]
    Parse(#[from] JspParseError),
}
