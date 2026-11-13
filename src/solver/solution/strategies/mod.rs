//! Implementaciones concretas de estrategias de solución.
//!
//! Todas las estrategias comparten la misma codificación: una secuencia con
//! una aparición por cada operación de cada trabajo. El decodificador de
//! `SolutionBuilder` traduce esa secuencia en una programación con tiempos de
//! inicio y fin, respetando la precedencia de los trabajos y la capacidad de
//! las máquinas.
//!
//! - `BruteForce`: enumera todas las permutaciones del multiconjunto.
//! - `Random`: evalúa muestras aleatorias y conserva la mejor.
//! - `Swap`: genera un único vecino mediante un intercambio.
//! - `LocalSearch`: baja hasta un mínimo local con primera mejora.
//! - `SimulatedAnnealing`: acepta empeoramientos con probabilidad
//!   decreciente para escapar de mínimos locales.

mod bruteforce;
mod local_search;
mod random;
mod simulated_annealing;
mod swap;

pub use bruteforce::BruteForce;
pub use local_search::LocalSearch;
pub use random::Random;
pub use simulated_annealing::SimulatedAnnealing;
pub use swap::Swap;
