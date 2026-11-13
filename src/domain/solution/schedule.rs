//! Programación completa de una instancia JSP.
//!
//! Una solución del JSP no es solo una secuencia de trabajos:
//! es una asignación de tiempos de inicio para cada operación.
//! En el modelo, `x_ij` representa el inicio de la operación del trabajo `j` en la máquina `i`,
//! y `C` es el makespan. Esta estructura guarda esa asignación y el valor de `C`.

use std::fmt::Display;

use super::ScheduleItem;

/// Programación completa obtenida para una instancia JSP.
///
/// Almacena una fila por operación programada ([`ScheduleItem`]) y el
/// makespan global `C`. La programación debe respetar la precedencia de cada
/// trabajo y la capacidad de cada máquina. Esas restricciones se garantizan
/// durante la decodificación de la secuencia.
pub struct Schedule {
    /// Makespan `C`: instante en que termina la última operación.
    ///
    /// Es la función objetivo que se minimiza.
    /// En el modelo, `C` es el máximo de los instantes de término de todos los trabajos.
    makespan: usize,

    /// Operaciones programadas con sus tiempos de inicio y fin.
    ///
    /// Cada elemento representa una operación `o_jr` y su variable `x_ij`
    /// (inicio). La cantidad de elementos debe ser igual al total de
    /// operaciones de la instancia.
    items: Vec<ScheduleItem>,
}

// Inicializadores
impl Schedule {
    /// Crea una programación a partir de su makespan y sus operaciones.
    pub fn new(makespan: usize, items: Vec<ScheduleItem>) -> Self {
        Self { makespan, items }
    }
}

impl Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Ordena una copia para mostrar las operaciones por trabajo y número
        // de operación. El orden de la programación no cambia, solo cambia la
        // presentación.
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
