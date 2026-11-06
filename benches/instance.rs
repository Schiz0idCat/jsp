#[derive(Clone, Copy)]
pub struct Instance {
    id: u8,
    name: &'static str,
    slug: &'static str,
    path: &'static str,
}

impl Instance {
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
    pub const fn id(&self) -> u8 {
        self.id
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn slug(&self) -> &'static str {
        self.slug
    }

    pub const fn path(&self) -> &'static str {
        self.path
    }
}
