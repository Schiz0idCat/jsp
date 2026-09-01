use thiserror::Error;

#[derive(Debug, Error)]
pub enum JspParseError {
    #[error("The content to parse is empty")]
    EmptyInput,

    #[error("Header must contain two integers (n_jobs n_machines)")]
    InvalidHeader,

    #[error("Failed to parse an integer in the input")]
    InvalidNumber(#[from] std::num::ParseIntError),

    #[error("Job line contains an odd number of values (expected machine duration pairs)")]
    OddOperationValues,
}
