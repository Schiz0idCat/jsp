//! Gestor que combina varias condiciones de parada.

use super::StopCondition;

/// Gestiona un conjunto de condiciones de parada para los solucionadores.
#[derive(Default)]
pub struct StopHandle {
    /// Condiciones que se consultan antes de continuar la búsqueda.
    conditions: Vec<Box<dyn StopCondition>>,
}

impl StopHandle {
    /// Inicializa un [`StopHandle`] sin condiciones.
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }

    /// Agrega una condición de parada y retorna el manejador actualizado.
    pub fn with_condition(mut self, condition: impl StopCondition + 'static) -> Self {
        self.conditions.push(Box::new(condition));
        self
    }

    /// Reinicia todas las condiciones de parada.
    pub fn start(&mut self) {
        for condition in &mut self.conditions {
            condition.reset();
        }
    }

    /// Indica si al menos una condición solicita detener la ejecución.
    pub fn should_stop(&self) -> bool {
        self.conditions
            .iter()
            .any(|condition| condition.should_stop())
    }
}
