pub struct Uuid {
    value: String,
}

impl Uuid {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
