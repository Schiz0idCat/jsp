use std::fmt::Display;

use super::ScheduleItem;

/// Representa la solución o programación completa obtenida para una instancia JSP.
///
/// Almacena la lista de operaciones programadas ([`ScheduleItem`]) y el makespan global
/// resultante (tiempo total para completar todos los trabajos).
pub struct Schedule {
    makespan: usize,
    items: Vec<ScheduleItem>,
}

// Inicializadores
impl Schedule {
    pub fn new(makespan: usize, items: Vec<ScheduleItem>) -> Self {
        Self { makespan, items }
    }
}

impl Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sorted_items = self.items.clone();
        sorted_items.sort_by_key(|item| (item.job(), item.op()));

        writeln!(f, "Optimal Makespan: {}\n", self.makespan)?;
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
