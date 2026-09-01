use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScheduleItem {
    job: usize,
    op: usize,
    machine: usize,
    start: usize,
    end: usize,
    duration: usize,
}

impl ScheduleItem {
    pub fn job(self) -> usize {
        self.job
    }

    pub fn op(self) -> usize {
        self.op
    }
}

impl ScheduleItem {
    pub fn new(
        job: usize,
        op: usize,
        machine: usize,
        start: usize,
        end: usize,
        duration: usize,
    ) -> Self {
        Self {
            job,
            op,
            machine,
            start,
            end,
            duration,
        }
    }
}

impl Display for ScheduleItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<6}{:<6}{:<10}{:<8}{:<8}{:<10}",
            self.job, self.op, self.machine, self.start, self.end, self.duration
        )
    }
}
