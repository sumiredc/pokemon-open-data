use crate::domain::model::pokemon::Pokemon;

pub struct PokemonOutput {
    data: Pokemon,
}

impl PokemonOutput {
    pub fn new(data: Pokemon) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &Pokemon {
        &self.data
    }
}
