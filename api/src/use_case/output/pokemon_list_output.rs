use crate::domain::model::pokemon::Pokemon;

pub struct PokemonListOutput {
    data: Vec<Pokemon>,
}

impl PokemonListOutput {
    pub fn new(data: Vec<Pokemon>) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &Vec<Pokemon> {
        &self.data
    }
}
