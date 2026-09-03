use clap::Parser;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Extrae los argumentos de la CLI.
#[derive(Parser, Debug)]
#[command(about = "Job Shop Scheduling Problem (JSP) Solver", long_about = None)]
pub struct Cli {
    /// Ruta al archivo con el problema.
    #[arg(short, long = "file", value_name = "FILE")]
    filepath: PathBuf,

    /// Tiempo límite opcional.
    #[arg(short, long, value_name = "SECONDS")]
    timeout: Option<u64>,
}

// Getters
impl Cli {
    pub fn filepath(&self) -> &Path {
        &self.filepath
    }

    pub fn timeout(&self) -> Option<Duration> {
        match self.timeout {
            Some(0) | None => None,
            Some(secs) => Some(Duration::from_secs(secs)),
        }
    }
}
