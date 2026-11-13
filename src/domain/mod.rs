//! Modelo de dominio del Job Shop Problem.
//!
//! El modelo del enunciado define:
//! - `J = {1, ..., n}`: conjunto de trabajos.
//! - `M = {1, ..., m}`: conjunto de máquinas.
//! - `O_j`: secuencia ordenada de operaciones del trabajo `j`.
//! - `p`: tiempo de procesamiento de cada operación.
//! - `x_ij`: instante de inicio de una operación.
//! - `C`: makespan, instante en que termina el último trabajo.
//!
//! Este módulo agrupa esas entidades ([`Job`](crate::domain::Job),
//! [`Operation`](crate::domain::Operation) y [`Jsp`](crate::domain::Jsp)),
//! los errores de lectura y las estructuras que representan una solución.
//!
//! Correspondencia entre el código y el modelo:
//! - `Jsp::n_jobs` -> `n = |J|`.
//! - `Jsp::n_machines` -> `m = |M|`.
//! - `Job::id` -> identificador visible del trabajo `j`.
//! - `Job::operations` -> secuencia `O_j`.
//! - `Operation::machine` -> máquina `o_jr`.
//! - `Operation::duration` -> tiempo de procesamiento `p`.
//! - `ScheduleItem::start` -> tiempo de inicio `x_ij`.
//! - `Schedule::makespan` -> makespan `C`.

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
