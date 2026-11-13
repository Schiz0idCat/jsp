//! Acumulación y resumen de los makespan de una instancia.

use std::fmt::Display;

/// Resultado agregado de las ejecuciones de una instancia.
pub struct BenchmarkResult {
    /// Nombre de la instancia a la que pertenecen las muestras.
    instance_name: &'static str,

    /// Makespan obtenido en cada ejecución.
    makespans: Vec<f64>,
}

impl BenchmarkResult {
    /// Crea un resultado si la lista de muestras no está vacía.
    pub fn new(instance_name: &'static str, makespans: Vec<f64>) -> Option<Self> {
        if makespans.is_empty() {
            None
        } else {
            Some(Self {
                instance_name,
                makespans,
            })
        }
    }

    /// Retorna la cantidad de ejecuciones registradas.
    pub fn len(&self) -> usize {
        self.makespans.len()
    }

    /// Indica si no se registraron ejecuciones.
    pub fn is_empty(&self) -> bool {
        self.makespans.is_empty()
    }

    /// Retorna el menor makespan registrado.
    pub fn min(&self) -> f64 {
        self.makespans.iter().cloned().fold(f64::INFINITY, f64::min)
    }

    /// Retorna el mayor makespan registrado.
    pub fn max(&self) -> f64 {
        self.makespans
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max)
    }

    /// Calcula la media aritmética de las muestras.
    pub fn mean(&self) -> f64 {
        let sum: f64 = self.makespans.iter().sum();
        sum / self.len() as f64
    }

    /// Calcula la desviación estándar muestral.
    pub fn std_dev(&self) -> f64 {
        if self.len() <= 1 {
            return 0.0;
        }

        let mean = self.mean();
        let variance: f64 = self
            .makespans
            .iter()
            .map(|value| (value - mean).powi(2))
            .sum::<f64>()
            / (self.len() - 1) as f64;

        variance.sqrt()
    }
}

impl Display for BenchmarkResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n--------------------------------------------------")?;
        writeln!(f, " RESULTADOS: Instancia {}", self.instance_name)?;
        writeln!(f, "--------------------------------------------------")?;
        writeln!(f, " Muestras tomadas        : {}", self.len())?;
        writeln!(f, " Mejor solución (Mínimo) : {:.0}", self.min())?;
        writeln!(f, " Peor solución (Máximo)  : {:.0}", self.max())?;
        writeln!(f, " Media                   : {:.2}", self.mean())?;
        writeln!(f, " Desviación Estándar     : {:.2}", self.std_dev())?;
        writeln!(f, "--------------------------------------------------\n")
    }
}
