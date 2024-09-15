pub struct Pokemon {
    number: u16,
    name: String,
}

impl Pokemon {
    pub fn new(number: u16, name: String) -> Self {
        Pokemon { number, name }
    }

    pub fn number(&self) -> &u16 {
        &self.number
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
