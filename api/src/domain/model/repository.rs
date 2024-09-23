use super::{pokemon::Pokemon, summary::Summary};

pub trait PokemonRepository {
    fn find_by_number(&self, number: u16) -> Option<Pokemon>;
    fn list(&self) -> Option<Vec<Pokemon>>;
}

pub trait SummaryRepository {
    fn r#type(&self) -> Option<Vec<Summary>>;
    fn generation(&self) -> Option<Vec<Summary>>;
    fn stats(&self) -> Option<Vec<Summary>>;
}
