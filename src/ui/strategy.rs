//! Estrategias de solución disponibles en la línea de comandos.

use clap::Subcommand;

/// Estrategias de solución y sus argumentos.
#[derive(Subcommand, Debug, Clone, Copy)]
pub enum Strategy {
    /// Búsqueda exhaustiva por permutaciones.
    Bruteforce,

    /// Muestreo aleatorio de secuencias.
    Random {
        /// Cantidad de secuencias aleatorias a evaluar.
        #[arg(short, long, default_value = "1")]
        samples: usize,
    },

    /// Generación de un vecino mediante un intercambio aleatorio.
    Swap,

    /// Búsqueda local por primera mejora.
    LocalSearch {
        /// Cantidad de soluciones iniciales a explorar.
        #[arg(short, long, default_value = "1")]
        samples: usize,
    },

    /// Metaheurística Simulated Annealing.
    SimulatedAnnealing {
        /// Temperatura inicial.
        #[arg(short = 'i', long, default_value = "1000.0")]
        t_initial: f64,

        /// Temperatura mínima.
        #[arg(short = 'm', long, default_value = "0.001")]
        t_min: f64,

        /// Tasa de enfriamiento.
        #[arg(short, long, default_value = "0.9995")]
        alpha: f64,
    },
}
