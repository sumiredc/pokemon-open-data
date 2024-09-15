use crate::domain::model::repository::PokemonRepository;

use super::output::pokemon_list_output::PokemonListOutput;

pub struct PokemonListUseCase<T>
where
    T: PokemonRepository,
{
    pokemon_repository: T,
}

impl<T: PokemonRepository> PokemonListUseCase<T> {
    pub fn new(pokemon_repository: T) -> Self {
        Self { pokemon_repository }
    }

    pub fn execute(&self) -> Result<PokemonListOutput, ()> {
        let data = self.pokemon_repository.list().unwrap_or_default();
        Ok(PokemonListOutput::new(data))
    }
}
