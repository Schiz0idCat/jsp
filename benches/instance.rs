//! Metadatos de las instancias usadas por el benchmark.

/// Describe una instancia de prueba y las rutas necesarias para reportarla.
#[derive(Clone, Copy)]
pub struct Instance {
    /// Identificador numérico de la instancia.
    id: u8,

    /// Nombre legible para la salida y los informes.
    name: &'static str,

    /// Nombre corto usado en los archivos CSV.
    slug: &'static str,

    /// Ruta del archivo de la instancia.
    path: &'static str,
}

impl Instance {
    /// Crea la descripción de una instancia.
    pub const fn new(id: u8, name: &'static str, slug: &'static str, path: &'static str) -> Self {
        Self {
            id,
            name,
            slug,
            path,
        }
    }
}

impl Instance {
    /// Retorna el identificador numérico.
    pub const fn id(&self) -> u8 {
        self.id
    }

    /// Retorna el nombre legible.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Retorna el nombre corto usado en los CSV.
    pub const fn slug(&self) -> &'static str {
        self.slug
    }

    /// Retorna la ruta del archivo de la instancia.
    pub const fn path(&self) -> &'static str {
        self.path
    }
}
