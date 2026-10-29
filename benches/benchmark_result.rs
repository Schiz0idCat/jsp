use std::fmt::Display;

pub struct BenchmarkResult {
    instance_name: &'static str,
    makespans: Vec<f64>,
}

impl BenchmarkResult {
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
    fn len(&self) -> usize {
        self.makespans.len()
    }

    /// Retorna el mejor valor (mínimo makespan).
    fn min(&self) -> f64 {
        self.makespans.iter().cloned().fold(f64::INFINITY, f64::min)
    }

    /// Retorna el peor valor (máximo makespan).
    fn max(&self) -> f64 {
        self.makespans
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max)
    }

    /// Calcula la media aritmética de las muestras.
    fn mean(&self) -> f64 {
        let sum: f64 = self.makespans.iter().sum();
        sum / self.len() as f64
    }

    /// Calcula la desviación estándar muestral.
    fn std_dev(&self) -> f64 {
        if self.len() <= 1 {
            return 0.0;
        }

        let mean = self.mean();
        let variance: f64 = self
            .makespans
            .iter()
            .map(|val| (val - mean).powi(2))
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
