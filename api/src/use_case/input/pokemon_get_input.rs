pub struct PokemonGetInput {
    number: u16,
}

impl PokemonGetInput {
    pub fn new(number: u16) -> Self {
        Self { number }
    }

    pub fn number(&self) -> &u16 {
        &self.number
    }
}
