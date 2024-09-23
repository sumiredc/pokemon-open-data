pub struct Summary {
    label: String,
    value: usize,
}

impl Summary {
    pub fn new(label: String, value: usize) -> Self {
        Self { label, value }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn value(&self) -> &usize {
        &self.value
    }
}
