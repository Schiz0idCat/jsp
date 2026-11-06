use std::fmt::{self, Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

/// Semilla utilizada para inicializar los generadores aleatorios.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RandomSeed(u64);

impl RandomSeed {
    /// Genera una semilla a partir de la fuente de entropía del sistema.
    pub fn generate() -> Self {
        Self(rand::rng().random())
    }

    /// Construye una semilla a partir de un valor conocido.
    pub const fn from_u64(value: u64) -> Self {
        Self(value)
    }

    /// Retorna el valor numérico de la semilla.
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Crea un generador determinista a partir de la semilla.
    ///
    /// Se retorna un generador nuevo en cada llamada para que dos ejecuciones
    /// con la misma semilla sigan exactamente la misma trayectoria.
    pub fn rng(self) -> StdRng {
        StdRng::seed_from_u64(self.0)
    }
}

impl Display for RandomSeed {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for RandomSeed {
    type Err = ParseIntError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.parse().map(Self)
    }
}
