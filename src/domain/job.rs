use std::fmt::Display;

use super::Operation;

pub struct Job {
    id: usize,
    operations: Vec<Operation>,
}

impl Job {
    pub fn new(id: usize, operations: Vec<Operation>) -> Self {
        Self { id, operations }
    }
}

impl Job {
    pub fn operations(&self) -> &[Operation] {
        &self.operations
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ops = self
            .operations
            .iter()
            .map(|op| format!("{:^10}", op.to_string()))
            .collect::<Vec<_>>()
            .join(" | ");

        write!(f, "Job {:<2} | {}", self.id, ops)
    }
}
