use serde::Serialize;

use crate::domain::model::{pokemon::Pokemon, r#type::Type};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonSchema {
    id: String,
    number: u16,
    name: String,
    english_name: String,
    type1: TypeSchema,
    type2: Option<TypeSchema>,
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
