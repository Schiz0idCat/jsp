use super::Solver;
use crate::domain::Jsp;
use crate::domain::solution::{Schedule, ScheduleItem};
use std::cmp;

pub struct BruteForce;

impl BruteForce {
    pub fn new() -> Self {
        Self
    }

    fn evaluate_makespan(&self, jsp: &Jsp, sequence: &[usize], max_machine_id: usize) -> usize {
        let mut job_end_time = vec![0; jsp.n_jobs()];
        let mut machine_end_time = vec![0; max_machine_id + 1];
        let mut job_op_index = vec![0; jsp.n_jobs()];

        for &job_idx in sequence {
            let job = &jsp.jobs()[job_idx];
            let op_idx = job_op_index[job_idx];

            let operation = &job.operations()[op_idx];
            let m_id = operation.machine();
            let duration = operation.duration();

            let start_time = cmp::max(job_end_time[job_idx], machine_end_time[m_id]);
            let end_time = start_time + duration;

            job_end_time[job_idx] = end_time;
            machine_end_time[m_id] = end_time;
            job_op_index[job_idx] += 1;
        }

        job_end_time.into_iter().max().unwrap_or(0)
    }

    fn build_schedule(&self, jsp: &Jsp, sequence: &[usize], max_machine_id: usize) -> Schedule {
        let mut job_end_time = vec![0; jsp.n_jobs()];
        let mut machine_end_time = vec![0; max_machine_id + 1];
        let mut job_op_index = vec![0; jsp.n_jobs()];

        let mut items = Vec::with_capacity(sequence.len());

        for &job_idx in sequence {
            let job = &jsp.jobs()[job_idx];
            let op_idx = job_op_index[job_idx];

            let operation = &job.operations()[op_idx];
            let m_id = operation.machine();
            let duration = operation.duration();

            let start_time = cmp::max(job_end_time[job_idx], machine_end_time[m_id]);
            let end_time = start_time + duration;

            job_end_time[job_idx] = end_time;
            machine_end_time[m_id] = end_time;
            job_op_index[job_idx] += 1;

            items.push(ScheduleItem::new(
                job.id(),
                op_idx + 1,
                m_id,
                start_time,
                end_time,
                duration,
            ));
        }

        let makespan = job_end_time.into_iter().max().unwrap_or(0);
        Schedule::new(makespan, items)
    }
}

impl Solver for BruteForce {
    fn solve(&self, jsp: &Jsp) -> Option<Schedule> {
        if jsp.jobs().is_empty() {
            return None;
        }

        let max_machine_id = jsp
            .jobs()
            .iter()
            .flat_map(|job| job.operations().iter())
            .map(|op| op.machine())
            .max()?;

        let mut base_sequence = Vec::new();
        for (idx, job) in jsp.jobs().iter().enumerate() {
            for _ in 0..job.operations().len() {
                base_sequence.push(idx);
            }
        }

        let mut best_makespan = usize::MAX;
        let mut best_sequence: Option<Vec<usize>> = None;

        let mut current_seq = base_sequence;

        if !current_seq.is_empty() {
            loop {
                let makespan = self.evaluate_makespan(jsp, &current_seq, max_machine_id);

                if makespan < best_makespan {
                    best_makespan = makespan;
                    best_sequence = Some(current_seq.clone());
                }

                if !next_permutation(&mut current_seq) {
                    break;
                }
            }
        }

        let best_seq = best_sequence?;
        Some(self.build_schedule(jsp, &best_seq, max_machine_id))
    }
}

fn next_permutation<T: Ord>(arr: &mut [T]) -> bool {
    if arr.len() <= 1 {
        return false;
    }
    let mut i = arr.len() - 1;
    while i > 0 && arr[i - 1] >= arr[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }
    let mut j = arr.len() - 1;
    while arr[j] <= arr[i - 1] {
        j -= 1;
    }
    arr.swap(i - 1, j);
    arr[i..].reverse();
    true
}
