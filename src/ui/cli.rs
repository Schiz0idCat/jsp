//! Definición de los argumentos de la línea de comandos.

use crate::solver::RandomSeed;
use crate::ui::Strategy;

use clap::Parser;
use std::num::NonZeroU64;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Argumentos de la interfaz de línea de comandos.
#[derive(Parser, Debug)]
#[command(about = "Job Shop Problem (JSP) Solver", long_about = None)]
pub struct Cli {
    /// Ruta al archivo con el problema.
    #[arg(short, long = "file", value_name = "FILE")]
    filepath: PathBuf,

    /// Tiempo límite opcional en segundos.
    #[arg(short, long, value_name = "SECONDS")]
    timeout: Option<u64>,

    /// Cantidad máxima de iteraciones de la búsqueda.
    #[arg(short = 'n', long = "iterations", value_name = "N", global = true)]
    iterations: Option<u64>,

    /// Semilla aleatoria para reproducir la ejecución.
    #[arg(long, value_name = "U64", global = true)]
    seed: Option<RandomSeed>,

    /// Subcomando que selecciona la estrategia de solución.
    #[command(subcommand)]
    strategy: Strategy,
}

// Getters
impl Cli {
    /// Retorna la ruta al archivo de la instancia.
    pub fn filepath(&self) -> &Path {
        &self.filepath
    }

    /// Retorna la duración máxima de ejecución, si se especificó.
    ///
    /// Un valor de `0` se interpreta como ausencia de límite.
    pub fn timeout(&self) -> Option<Duration> {
        match self.timeout {
            Some(0) | None => None,
            Some(secs) => Some(Duration::from_secs(secs)),
        }
    }

    /// Retorna la semilla indicada por el usuario, si existe.
    pub fn seed(&self) -> Option<RandomSeed> {
        self.seed
    }

    /// Retorna la cantidad máxima de iteraciones, si se especificó.
    ///
    /// Un valor de `0` se interpreta como ausencia de límite.
    pub fn iterations(&self) -> Option<NonZeroU64> {
        self.iterations.and_then(NonZeroU64::new)
    }

    /// Retorna la estrategia seleccionada para resolver la instancia.
    pub fn strategy(&self) -> Strategy {
        self.strategy
    }
}
