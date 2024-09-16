use crate::domain::model::pokemon::Pokemon;

use super::schema::PokemonSchema;

pub struct PokemonReponder {}

impl PokemonReponder {
    pub fn create(&self, pokemon: &Pokemon) -> PokemonSchema {
        PokemonSchema::new(pokemon)
    }

    pub fn create_all(&self, data: &Vec<Pokemon>) -> Vec<PokemonSchema> {
        data.iter()
            .map(|pokemon| PokemonSchema::new(pokemon))
            .collect()
    }
}
