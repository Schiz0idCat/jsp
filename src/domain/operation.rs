use std::fmt::Display;

pub struct Operation {
    machine: usize,
    duration: usize,
}

impl Operation {
    pub fn new(machine: usize, duration: usize) -> Self {
        Self { machine, duration }
    }
}

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
