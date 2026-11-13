//! Utilidades compartidas para evaluar secuencias y construir programaciones.

use crate::domain::Jsp;
use crate::domain::solution::{Schedule, ScheduleItem};

use std::cmp;

/// Lógica común para que las estrategias construyan una solución de [`Jsp`].
///
/// Una secuencia es un vector de índices de trabajos. Cada aparición de un
/// índice representa la siguiente operación pendiente de ese trabajo.
pub trait SolutionBuilder {
    /// Calcula el makespan de una secuencia de operaciones.
    ///
    /// La secuencia debe contener exactamente una aparición por cada operación
    /// de la instancia y respetar la multiplicidad de cada trabajo.
    ///
    /// # Panics
    ///
    /// Puede entrar en pánico si la secuencia hace referencia a un trabajo
    /// inexistente o contiene más operaciones de las que tiene un trabajo.
    fn evaluate_makespan(&self, jsp: &Jsp, sequence: &[usize], max_machine_id: usize) -> usize {
        // Instante en que cada trabajo queda libre.
        let mut job_end_time = vec![0; jsp.n_jobs()];

        // Instante en que cada máquina queda libre.
        let mut machine_end_time = vec![0; max_machine_id + 1];

        // Índice de la próxima operación pendiente de cada trabajo.
        let mut job_op_index = vec![0; jsp.n_jobs()];

        for &job_idx in sequence {
            let job = &jsp.jobs()[job_idx];
            let op_idx = job_op_index[job_idx];

            // Datos de la operación que corresponde ejecutar.
            let operation = &job.operations()[op_idx];
            let m_id = operation.machine();
            let duration = operation.duration();

            // La operación comienza cuando el trabajo y la máquina están libres.
            let start_time = cmp::max(job_end_time[job_idx], machine_end_time[m_id]);
            let end_time = start_time + duration;

            job_end_time[job_idx] = end_time;
            machine_end_time[m_id] = end_time;
            job_op_index[job_idx] += 1;
        }

        // El makespan es el mayor instante de término entre todos los trabajos.
        job_end_time.into_iter().max().unwrap_or(0)
    }

    /// Construye una [`Schedule`] a partir de una secuencia válida.
    ///
    /// # Panics
    ///
    /// Puede entrar en pánico si la secuencia hace referencia a un trabajo
    /// inexistente o contiene más operaciones de las que tiene un trabajo.
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
