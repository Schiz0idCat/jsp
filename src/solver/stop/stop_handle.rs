use super::StopCondition;

/// Gestor de condiciones de parada para los solucionadores.
#[derive(Default)]
pub struct StopHandle {
    conditions: Vec<Box<dyn StopCondition>>,
}

impl StopHandle {
    /// Inicializa un [`StopHandle`]
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }

    /// Builder para añadir condiciones de parada.
    pub fn with_condition(mut self, condition: impl StopCondition + 'static) -> Self {
        self.conditions.push(Box::new(condition));
        self
    }

    /// Inicializa todas las condiciones reseteando su estado interno.
    pub fn start(&mut self) {
        for condition in &mut self.conditions {
            condition.reset();
        }
    }

    /// Evalua si al menos una de las condiciones solicita una detención.
    pub fn should_stop(&self) -> bool {
        self.conditions.iter().any(|c| c.should_stop())
    }
}
