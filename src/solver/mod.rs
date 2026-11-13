//! Estrategias de solución y condiciones de parada.

mod seed;

/// Estrategias de solución y utilidades compartidas.
pub mod solution;

/// Condiciones de parada de las estrategias.
pub mod stop;

/// Almacenamiento de la semilla para reproducir resultados.
pub use seed::RandomSeed;
