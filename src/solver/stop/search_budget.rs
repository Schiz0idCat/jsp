//! Política de presupuesto de búsqueda basada en cantidad de iteraciones.

use std::num::NonZeroU64;

/// Define si la búsqueda tiene un límite de iteraciones.
///
/// Esta estructura representa solo la política del presupuesto. No almacena
/// cuántas iteraciones se han consumido: ese estado mutable pertenece a
/// [`SearchControl`](super::SearchControl).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SearchBudget {
    /// La búsqueda no tiene límite de iteraciones.
    #[default]
    Unlimited,

    /// La búsqueda se detiene después de `max_iterations` iteraciones.
    Limited(NonZeroU64),
}

impl SearchBudget {
    /// Crea una política de presupuesto a partir de un límite opcional.
    ///
    /// `None` representa una búsqueda sin límite.
    pub const fn new(max_iterations: Option<NonZeroU64>) -> Self {
        match max_iterations {
            Some(max_iterations) => Self::Limited(max_iterations),
            None => Self::Unlimited,
        }
    }

    /// Indica si todavía se permite ejecutar una iteración.
    ///
    /// `iterations` es la cantidad de iteraciones ya ejecutadas.
    pub fn allows(self, iterations: u64) -> bool {
        match self {
            Self::Unlimited => true,
            Self::Limited(max_iterations) => iterations < max_iterations.get(),
        }
    }
}
