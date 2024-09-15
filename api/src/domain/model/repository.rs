use super::pokemon::Pokemon;

pub trait PokemonRepository {
    fn find_by_number(&self, number: u16) -> Option<Pokemon>;
    fn list(&self) -> Option<Vec<Pokemon>>;
}
