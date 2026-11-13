//! Utilidades compartidas para evaluar secuencias y construir programaciones.
//!
//! # Decodificación de una secuencia
//!
//! Las estrategias no manipulan directamente los tiempos `x_ij`, si no que
//! manipulan una secuencia de identificadores de trabajo. Esta utilidad traduce
//! esa secuencia a una programación concreta:
//!
//! 1. Recorre la secuencia de izquierda a derecha.
//! 2. Cada aparición de un trabajo corresponde a la siguiente operación que
//!    debe programarse para ese trabajo.
//! 3. Programa esa operación lo antes posible, respetando:
//!    - la precedencia del trabajo (`o_j,r` después de `o_j,r-1`).
//!    - la disponibilidad de la máquina (`o_jr` no puede solaparse con otra
//!      operación en la misma máquina).
//!
//! El resultado es una asignación de tiempos de inicio `x_ij` y un makespan
//! `C`, que es el máximo instante de término entre todos los trabajos.

use crate::domain::Jsp;
use crate::domain::solution::{Schedule, ScheduleItem};

use std::cmp;

/// Lógica común para que las estrategias construyan una solución de [`Jsp`].
///
/// Una secuencia es un vector de índices de trabajos. Cada aparición de un
/// índice representa la siguiente operación pendiente de ese trabajo.
pub trait SolutionBuilder {
    /// Calcula el makespan `C` de una secuencia de operaciones.
    ///
    /// La secuencia debe contener exactamente una aparición por cada operación
    /// de la instancia y respetar la multiplicidad de cada trabajo.
    ///
    /// # Panics
    ///
    /// Puede entrar en pánico si la secuencia hace referencia a un trabajo
    /// inexistente o contiene más operaciones de las que tiene un trabajo.
    fn evaluate_makespan(&self, jsp: &Jsp, sequence: &[usize], max_machine_id: usize) -> usize {
        // `job_end_time[j]` es el instante en que el trabajo `j` queda libre,
        // es decir, el término de su última operación programada. Corresponde
        // al lado derecho de la restricción de precedencia del modelo:
        // `x_{o_jr,j} >= x_{o_j(r-1),j} + p`.
        let mut job_end_time = vec![0; jsp.n_jobs()];

        // `machine_end_time[m]` es el instante en que la máquina `m` queda
        // libre. Impide que dos operaciones se ejecuten simultáneamente en la
        // misma máquina (restricciones de no solapamiento del modelo).
        let mut machine_end_time = vec![0; max_machine_id + 1];

        // `job_op_index[j]` es la cantidad de operaciones del trabajo `j` que
        // ya se programaron. Por lo tanto, es el índice `r - 1` de la próxima
        // operación `o_jr` que debe programarse.
        let mut job_op_index = vec![0; jsp.n_jobs()];

        for &job_idx in sequence {
            // `job_idx` es la posición 0-based del trabajo en `jsp.jobs()`,
            // no el identificador visible `Job::id`.
            let job = &jsp.jobs()[job_idx];
            let op_idx = job_op_index[job_idx];

            // Datos de la operación que corresponde ejecutar:
            // `m_id` es la máquina `o_jr` y `duration` es su tiempo `p`.
            let operation = &job.operations()[op_idx];
            let m_id = operation.machine();
            let duration = operation.duration();

            // La operación comienza cuando el trabajo y la máquina están
            // libres. El máximo aplica simultáneamente la precedencia del
            // trabajo y la capacidad de la máquina.
            let start_time = cmp::max(job_end_time[job_idx], machine_end_time[m_id]);
            let end_time = start_time + duration;

            // Actualiza la disponibilidad para las siguientes operaciones.
            job_end_time[job_idx] = end_time;
            machine_end_time[m_id] = end_time;
            job_op_index[job_idx] += 1;
        }

        // El makespan `C` es el mayor instante de término entre todos los
        // trabajos. Si no hay operaciones, se retorna 0.
        job_end_time.into_iter().max().unwrap_or(0)
    }

    /// Construye una [`Schedule`] a partir de una secuencia válida.
    ///
    /// A diferencia de [`SolutionBuilder::evaluate_makespan`], este método
    /// conserva cada operación programada para poder mostrar la tabla de
    /// tiempos de inicio y fin.
    ///
    /// # Panics
    ///
    /// Puede entrar en pánico si la secuencia hace referencia a un trabajo
    /// inexistente o contiene más operaciones de las que tiene un trabajo.
    fn build_schedule(&self, jsp: &Jsp, sequence: &[usize], max_machine_id: usize) -> Schedule {
        // Los tres arreglos tienen el mismo significado que en
        // `evaluate_makespan`: disponibilidad de trabajos, disponibilidad de
        // máquinas y próxima operación pendiente de cada trabajo.
        let mut job_end_time = vec![0; jsp.n_jobs()];
        let mut machine_end_time = vec![0; max_machine_id + 1];
        let mut job_op_index = vec![0; jsp.n_jobs()];

        // Un `ScheduleItem` por cada operación programada.
        let mut items = Vec::with_capacity(sequence.len());

        for &job_idx in sequence {
            let job = &jsp.jobs()[job_idx];
            let op_idx = job_op_index[job_idx];

            let operation = &job.operations()[op_idx];
            let m_id = operation.machine();
            let duration = operation.duration();

            // Misma regla de programación que en `evaluate_makespan`.
            let start_time = cmp::max(job_end_time[job_idx], machine_end_time[m_id]);
            let end_time = start_time + duration;

            job_end_time[job_idx] = end_time;
            machine_end_time[m_id] = end_time;
            job_op_index[job_idx] += 1;

            // `op_idx + 1` porque el modelo numera las operaciones desde 1.
            // `job.id()` es el identificador visible 1-based del trabajo.
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
