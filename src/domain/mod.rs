//! Modelo de dominio del Job Shop Problem.
//!
//! Agrupa las entidades del problema ([`Job`](crate::domain::Job),
//! [`Operation`](crate::domain::Operation) y [`Jsp`](crate::domain::Jsp)),
//! sus errores de lectura y las estructuras que representan una solución.

/// Errores de lectura y parseo de instancias.
pub mod errors;

mod job;
mod jsp;
mod operation;

/// Programaciones y sus elementos.
pub mod solution;

pub use job::Job;
pub use jsp::Jsp;
pub use operation::Operation;
