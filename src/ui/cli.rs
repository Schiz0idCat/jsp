use crate::ui::Strategy;

use clap::Parser;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Extrae los argumentos de la CLI.
#[derive(Parser, Debug)]
#[command(about = "Job Shop Problem (JSP) Solver", long_about = None)]
pub struct Cli {
    /// Ruta al archivo con el problema.
    #[arg(short, long = "file", value_name = "FILE")]
    filepath: PathBuf,

    /// Tiempo límite opcional.
    #[arg(short, long, value_name = "SECONDS")]
    timeout: Option<u64>,

    /// Subcomando para elegir la estrategia de solución.
    #[command(subcommand)]
    strategy: Strategy,
}

// Getters
impl Cli {
    /// Retorna el path al archivo de la instancia.
    pub fn filepath(&self) -> &Path {
        &self.filepath
    }

    /// Retorna la duración máxima de ejecución, si la hay.
    pub fn timeout(&self) -> Option<Duration> {
        match self.timeout {
            Some(0) | None => None,
            Some(secs) => Some(Duration::from_secs(secs)),
        }
    }

    /// Retorna la estrategia para resolver la instancia.
    pub fn strategy(&self) -> Strategy {
        self.strategy
    }
}
