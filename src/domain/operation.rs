use std::fmt::Display;

/// Representa una operación individual dentro de un [`Job`](super::Job)
pub struct Operation {
    machine: usize,
    duration: usize,
}

// Inicializadores
impl Operation {
    pub fn new(machine: usize, duration: usize) -> Self {
        Self { machine, duration }
    }
}

// Getters
impl Operation {
    pub fn machine(&self) -> usize {
        self.machine
    }

    pub fn duration(&self) -> usize {
        self.duration
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "M{} ({}h)", self.machine, self.duration)
    }
}
