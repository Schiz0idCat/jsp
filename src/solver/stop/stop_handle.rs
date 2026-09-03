use super::StopCondition;

#[derive(Default)]
pub struct StopHandle {
    conditions: Vec<Box<dyn StopCondition>>,
}

impl StopHandle {
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }

    pub fn with_condition(mut self, condition: impl StopCondition + 'static) -> Self {
        self.conditions.push(Box::new(condition));
        self
    }

    pub fn start(&mut self) {
        for condition in &mut self.conditions {
            condition.reset();
        }
    }

    pub fn should_stop(&self) -> bool {
        self.conditions.iter().any(|c| c.should_stop())
    }
}
