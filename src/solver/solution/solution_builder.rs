use crate::domain::Jsp;
use crate::domain::solution::{Schedule, ScheduleItem};

use std::cmp;

/// Lógica para que los Solvers puedan construir la solución de una instancia de [`Jsp`].
pub trait SolutionBuilder {
    /// Calcula el makespan de una seciencia de operaciones.
    fn evaluate_makespan(&self, jsp: &Jsp, sequence: &[usize], max_machine_id: usize) -> usize {
        let mut job_end_time = vec![0; jsp.n_jobs()]; // termino de los trabajos
        let mut machine_end_time = vec![0; max_machine_id + 1]; // cuando se libera cada máquina
        let mut job_op_index = vec![0; jsp.n_jobs()]; // operaciones para cada trabajo

        for &job_idx in sequence {
            let job = &jsp.jobs()[job_idx];
            let op_idx = job_op_index[job_idx]; // obtiene la operación pendiente

            // metadatos de la operación
            let operation = &job.operations()[op_idx];
            let m_id = operation.machine();
            let duration = operation.duration();

            // el start time es cuando se libere el trabajo o la máquina, lo que pase último
            let start_time = cmp::max(job_end_time[job_idx], machine_end_time[m_id]);
            let end_time = start_time + duration;

            job_end_time[job_idx] = end_time;
            machine_end_time[m_id] = end_time;
            job_op_index[job_idx] += 1;
        }

        job_end_time.into_iter().max().unwrap_or(0)
    }

    /// Contruye un [`Schedule`] a partir de una secuencia válida.
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
