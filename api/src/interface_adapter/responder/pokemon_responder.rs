use serde::Serialize;

use crate::domain::model::pokemon::Pokemon;

pub struct PokemonReponder {}

impl PokemonReponder {
    pub fn create(&self, pokemon: &Pokemon) -> PokemonSchema {
        PokemonSchema::new(pokemon)
    }

    pub fn create_all(&self, data: &Vec<Pokemon>) -> PokemonListSchema {
        PokemonListSchema {
            code: 200,
            message: String::from("Success"),
            data: data
                .iter()
                .map(|pokemon| PokemonSchema::new(pokemon))
                .collect::<Vec<PokemonSchema>>(),
        }
    }
}

#[derive(Serialize)]
pub struct PokemonListSchema {
    code: u16,
    message: String,
    data: Vec<PokemonSchema>,
}

#[derive(Serialize)]
pub struct PokemonSchema {
    number: u16,
    name: String,
}

impl PokemonSchema {
    pub fn new(data: &Pokemon) -> Self {
        Self {
            number: data.number().clone(),
            name: data.name().to_string(),
        }
    }
}
