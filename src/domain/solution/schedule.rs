//! Programación completa de una instancia JSP.

use std::fmt::Display;

use super::ScheduleItem;

/// Solución o programación completa obtenida para una instancia JSP.
///
/// Almacena las operaciones programadas ([`ScheduleItem`]) y el makespan
/// global resultante (tiempo total para completar todos los trabajos).
pub struct Schedule {
    /// Makespan de la programación.
    makespan: usize,

    /// Operaciones programadas con sus tiempos de inicio y fin.
    items: Vec<ScheduleItem>,
}

// Inicializadores
impl Schedule {
    /// Crea una programación a partir de su makespan y sus elementos.
    pub fn new(makespan: usize, items: Vec<ScheduleItem>) -> Self {
        Self { makespan, items }
    }
}

impl Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Ordena una copia para mostrar las operaciones por trabajo y número de operación.
        let mut sorted_items = self.items.clone();
        sorted_items.sort_by_key(|item| (item.job(), item.op()));

        writeln!(f, "Best Makespan: {}\n", self.makespan)?;
        writeln!(
            f,
            "{:<6}{:<6}{:<10}{:<8}{:<8}{:<10}",
            "Job", "Op", "Machine", "Start", "End", "Duration"
        )?;
        writeln!(f, "{}", "-".repeat(50))?;

        for item in sorted_items {
            writeln!(f, "{}", item)?;
        }

        Ok(())
    }
}
