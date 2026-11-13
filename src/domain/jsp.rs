//! Define una instancia completa del Job Shop Problem y su lectura.

use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use super::errors::{JspFileError, JspParseError};
use super::solution::Schedule;
use super::{Job, Operation};
use crate::solver::solution::Solver;
use crate::solver::stop::StopHandle;

/// Instancia completa del Job Shop Problem.
///
/// Contiene las dimensiones del problema (cantidad de trabajos y de máquinas)
/// y la lista de trabajos con sus operaciones.
pub struct Jsp {
    /// Cantidad de trabajos de la instancia.
    n_jobs: usize,

    /// Cantidad de máquinas de la instancia.
    n_machines: usize,

    /// Trabajos que componen la instancia.
    jobs: Vec<Job>,
}

// Inicializadores
impl Jsp {
    /// Crea una instancia a partir de sus dimensiones y sus trabajos.
    pub fn new(n_jobs: usize, n_machines: usize, jobs: Vec<Job>) -> Self {
        Self {
            n_jobs,
            n_machines,
            jobs,
        }
    }

    /// Carga una instancia desde un archivo de texto.
    ///
    /// # Errors
    ///
    /// Retorna [`JspFileError`] si el archivo no se puede leer o si su
    /// contenido no tiene el formato esperado.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, JspFileError> {
        let content = fs::read_to_string(path)?;
        let jsp: Jsp = content.parse()?;
        Ok(jsp)
    }
}

// Getters
impl Jsp {
    /// Retorna la cantidad de trabajos.
    pub fn n_jobs(&self) -> usize {
        self.n_jobs
    }

    /// Retorna la cantidad de máquinas.
    pub fn n_machines(&self) -> usize {
        self.n_machines
    }

    /// Retorna los trabajos de la instancia.
    pub fn jobs(&self) -> &[Job] {
        &self.jobs
    }
}

// Operaciones para resolver el problema
impl Jsp {
    /// Resuelve el problema con la estrategia indicada.
    ///
    /// Retorna la mejor programación encontrada o `None` si la estrategia
    /// no produce una solución.
    pub fn solve<S: Solver>(&self, solver: S) -> Option<Schedule> {
        solver.solve(self)
    }

    /// Resuelve el problema con una estrategia y un criterio de detención.
    ///
    /// Cuando se solicita la detención, retorna la mejor solución encontrada
    /// hasta ese momento.
    pub fn solve_with_stop<S: Solver>(&self, solver: S, stop: &mut StopHandle) -> Option<Schedule> {
        solver.solve_with_stop(self, stop)
    }
}

impl Display for Jsp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Calcula el ancho de la cabecera a partir del trabajo más largo.
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

// Habilita el método `.parse()` para construir una instancia desde texto.
impl FromStr for Jsp {
    type Err = JspParseError;

    /// Interpreta el formato de texto de una instancia JSP.
    ///
    /// La primera línea contiene `n_jobs n_machines`.
    /// Cada línea siguiente contiene pares `máquina duración` para un trabajo.
    ///
    /// # Errors
    ///
    /// Retorna [`JspParseError`] si la entrada está vacía, la cabecera es
    /// inválida, un número no se puede convertir o una línea tiene una
    /// cantidad impar de valores.
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
            // Cada línea de trabajo debe contener pares máquina-duración.
            let values: Vec<usize> = line
                .split_whitespace()
                .map(|token| token.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()?;

            if !values.len().is_multiple_of(2) {
                return Err(JspParseError::OddOperationValues);
            }

            let operations = values
                .as_chunks::<2>()
                .0
                .iter()
                .map(|chunk| Operation::new(chunk[0], chunk[1]))
                .collect();

            jobs.push(Job::new(idx + 1, operations));
        }

        Ok(Jsp::new(n_jobs, n_machines, jobs))
    }
}
