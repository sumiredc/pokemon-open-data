use crate::domain::entity::{stats::Stats, uuid::Uuid};

use super::{generation::Generation, r#type::Type};

pub struct Pokemon {
    id: Uuid,
    number: u16,
    name: String,
    english_name: String,
    type1: Type,
    type2: Option<Type>,
    generation: Generation,
    stats: Stats,
}

impl Pokemon {
    pub fn new(
        id: String,
        number: u16,
        name: String,
        english_name: String,
        type1: Type,
        type2: Option<Type>,
        generation: Generation,
        stats: Stats,
    ) -> Self {
        Self {
            id: Uuid::new(id),
            number,
            name,
            english_name,
            type1,
            type2,
            generation,
            stats,
        }
    }

    pub fn id(&self) -> &str {
        self.id.value()
    }

    pub fn number(&self) -> &u16 {
        &self.number
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn english_name(&self) -> &str {
        &self.english_name
    }

    pub fn type1(&self) -> &Type {
        &self.type1
    }

    pub fn type2(&self) -> &Option<Type> {
        &self.type2
    }

    pub fn generation(&self) -> &Generation {
        &self.generation
    }

    pub fn stats(&self) -> &Stats {
        &self.stats
    }
}
