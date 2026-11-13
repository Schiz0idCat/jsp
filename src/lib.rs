//! Biblioteca del solucionador de Job Shop Problem (JSP).
//!
//! El código se organiza en tres capas:
//! - [`domain`]: modelo del problema y de las programaciones.
//! - [`solver`]: estrategias de solución y condiciones de parada.
//! - [`ui`]: interfaz de línea de comandos y orquestación de la ejecución.

/// Modelo del problema: instancias, trabajos, operaciones y programaciones.
pub mod domain;

/// Estrategias de solución y mecanismos de parada.
pub mod solver;

/// Interfaz de usuario y construcción de las estrategias.
pub mod ui;
