pub struct Instance {
    name: &'static str,
    path: &'static str,
}

impl Instance {
    pub fn new(name: &'static str, path: &'static str) -> Self {
        Self { name, path }
    }
}

impl Instance {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn path(&self) -> &'static str {
        self.path
    }
}
