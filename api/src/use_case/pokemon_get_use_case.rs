use crate::domain::model::repository::PokemonRepository;

use super::{input::pokemon_get_input::PokemonGetInput, output::pokemon_output::PokemonOutput};

pub struct PokemonGetUseCase<T>
where
    T: PokemonRepository,
{
    pokemon_repository: T,
}

impl<T: PokemonRepository> PokemonGetUseCase<T> {
    pub fn new(pokemon_repository: T) -> Self {
        Self { pokemon_repository }
    }

    pub fn execute(&self, input: PokemonGetInput) -> Result<PokemonOutput, String> {
        let res = self.pokemon_repository.find_by_number(*input.number());
        match res {
            Some(pokemon) => Ok(PokemonOutput::new(pokemon)),
            None => Err(format!("Not Found.")),
        }
    }
}
