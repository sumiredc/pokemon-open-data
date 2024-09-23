use serde::Serialize;

use crate::domain::{
    entity::stats::Stats,
    model::{generation::Generation, pokemon::Pokemon, r#type::Type, summary::Summary},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonSchema {
    id: String,
    number: u16,
    name: String,
    english_name: String,
    type1: TypeSchema,
    type2: Option<TypeSchema>,
    generation: GenerationSchema,
    stats: StatsSchema,
}

impl PokemonSchema {
    pub fn new(data: &Pokemon) -> Self {
        Self {
            id: data.id().to_string(),
            number: *data.number(),
            name: data.name().to_string(),
            english_name: data.english_name().to_string(),
            type1: TypeSchema::new(data.type1()),
            type2: data.type2().as_ref().map(|type2| TypeSchema::new(type2)),
            generation: GenerationSchema::new(data.generation()),
            stats: StatsSchema::new(data.stats()),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeSchema {
    id: String,
    name: String,
}

impl TypeSchema {
    pub fn new(data: &Type) -> Self {
        Self {
            id: data.id().to_string(),
            name: data.name().to_string(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationSchema {
    id: String,
    value: u8,
}

impl GenerationSchema {
    pub fn new(data: &Generation) -> Self {
        Self {
            id: data.id().to_string(),
            value: data.value().clone(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SummarySchema {
    label: String,
    value: usize,
}

impl SummarySchema {
    pub fn new(data: &Summary) -> Self {
        Self {
            label: data.label().to_string(),
            value: *data.value(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsSchema {
    hp: u8,
    attack: u8,
    defense: u8,
    sp_attack: u8,
    sp_defense: u8,
    speed: u8,
}

impl StatsSchema {
    pub fn new(data: &Stats) -> Self {
        Self {
            hp: *data.hp(),
            attack: *data.attack(),
            defense: *data.defense(),
            sp_attack: *data.sp_attack(),
            sp_defense: *data.sp_defense(),
            speed: *data.speed(),
        }
    }
}
