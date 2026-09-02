use std::time::Duration;
use std::{fmt::Display, fs, path::Path, str::FromStr};

use super::errors::{JspFileError, JspParseError};
use super::solution::Schedule;
use super::{Job, Operation};
use crate::solver::Solver;

pub struct Jsp {
    n_jobs: usize,
    n_machines: usize,
    jobs: Vec<Job>,
}

impl Jsp {
    pub fn new(n_jobs: usize, n_machines: usize, jobs: Vec<Job>) -> Self {
        Self {
            n_jobs,
            n_machines,
            jobs,
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, JspFileError> {
        let content = fs::read_to_string(path)?;
        let jsp: Jsp = content.parse()?;
        Ok(jsp)
    }
}

impl Jsp {
    pub fn n_jobs(&self) -> usize {
        self.n_jobs
    }

    pub fn n_machines(&self) -> usize {
        self.n_machines
    }

    pub fn jobs(&self) -> &[Job] {
        &self.jobs
    }
}

impl Jsp {
    pub fn solve<S: Solver>(&self, solver: S) -> Option<Schedule> {
        solver.solve(self)
    }

    pub fn solve_with_timeout<S: Solver>(
        &self,
        solver: S,
        timeout: Option<Duration>,
    ) -> Option<Schedule> {
        solver.solve_with_timeout(self, timeout)
    }
}

impl Display for Jsp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_ops = self
            .jobs
            .iter()
            .map(|job| job.operations().len())
            .max()
            .unwrap_or(0);

        let ops_header = (1..=max_ops)
            .map(|i| format!("{:^10}", format!("Op {i}")))
            .collect::<Vec<_>>()
            .join(" | ");

        let title = format!(
            "JSP Instance [{} Jobs x {} Machines]",
            self.n_jobs, self.n_machines
        );

        let header = format!("{:<6} | {}", "", ops_header);
        let divider = "-".repeat(header.len());

        writeln!(f, "{title}")?;
        writeln!(f, "{divider}")?;
        writeln!(f, "{header}")?;
        writeln!(f, "{divider}")?;

        for job in &self.jobs {
            writeln!(f, "{job}")?;
        }

        write!(f, "{divider}")
    }
}

impl FromStr for Jsp {
    type Err = JspParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut non_empty_lines = s.lines().map(str::trim).filter(|line| !line.is_empty());

        let header = non_empty_lines.next().ok_or(JspParseError::EmptyInput)?;
        let mut header_parts = header.split_whitespace();

        let n_jobs: usize = header_parts
            .next()
            .ok_or(JspParseError::InvalidHeader)?
            .parse()?;

        let n_machines: usize = header_parts
            .next()
            .ok_or(JspParseError::InvalidHeader)?
            .parse()?;

        let mut jobs = Vec::new();

        for (idx, line) in non_empty_lines.enumerate() {
            let values: Vec<usize> = line
                .split_whitespace()
                .map(|token| token.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()?;

            if values.len() % 2 != 0 {
                return Err(JspParseError::OddOperationValues);
            }

            let operations = values
                .chunks_exact(2)
                .map(|chunk| Operation::new(chunk[0], chunk[1]))
                .collect();

            jobs.push(Job::new(idx + 1, operations));
        }

        Ok(Jsp::new(n_jobs, n_machines, jobs))
    }
}
