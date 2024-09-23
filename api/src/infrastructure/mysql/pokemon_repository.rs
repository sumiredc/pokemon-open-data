use mysql::{
    params,
    prelude::{FromRow, Queryable},
    Pool,
};

use crate::domain::{
    entity::stats::Stats,
    model::{
        generation::Generation, pokemon::Pokemon, r#type::Type, repository::PokemonRepository,
    },
};

pub struct PokemonRepositoryImpl {
    pub pool: Box<Pool>,
}

#[derive(FromRow)]
struct PokemonSchema {
    pokemon_id: String,
    number: u16,
    name: String,
    english_name: String,
    type1_id: String,
    type1_name: String,
    type2_id: Option<String>,
    type2_name: Option<String>,
    generation_id: String,
    generation_value: u8,
    hp: u8,
    attack: u8,
    defense: u8,
    sp_attack: u8,
    sp_defense: u8,
    speed: u8,
}

impl PokemonSchema {
    pub fn to_model(&self) -> Pokemon {
        Pokemon::new(
            self.pokemon_id.clone(),
            self.number,
            self.name.clone(),
            self.english_name.clone(),
            Type::new(self.type1_id.clone(), self.type1_name.clone()),
            match (self.type2_id.clone(), self.type2_name.clone()) {
                (Some(type_id), Some(name)) => Some(Type::new(type_id, name)),
                _ => None,
            },
            Generation::new(self.generation_id.clone(), self.generation_value.clone()),
            Stats::new(
                self.hp,
                self.attack,
                self.defense,
                self.sp_attack,
                self.sp_defense,
                self.speed,
            ),
        )
    }
}

impl PokemonRepository for PokemonRepositoryImpl {
    ///
    /// ポケモン 詳細
    ///
    fn find_by_number(&self, number: u16) -> Option<Pokemon> {
        match self.pool.get_conn() {
            Ok(mut conn) => {
                let query = r#"
                SELECT 
                    pokemon.pokemon_id, 
                    pokemon.number, 
                    pokemon.name,
                    pokemon.english_name,
                    pokemon.type1_id,
                    type1.name AS type1_name,
                    pokemon.type2_id,
                    type2.name AS type2_name,
                    generation.generation_id,
                    generation.value AS generation_value,
                    pokemon.hp,
                    pokemon.attack,
                    pokemon.defense,
                    pokemon.sp_attack,
                    pokemon.sp_defense,
                    pokemon.speed
                FROM 
                    pokemon
                    INNER JOIN type AS type1 ON type1.type_id = pokemon.type1_id
                    LEFT JOIN type AS type2 ON type2.type_id = pokemon.type2_id
                    INNER JOIN generation ON generation.generation_id = pokemon.generation_id
                WHERE 
                    number = :number
                "#;
                match conn.exec_first::<PokemonSchema, _, _>(query, params! { "number" => number })
                {
                    Ok(op) => op.map(|schema| schema.to_model()),
                    Err(err) => {
                        eprintln!("{}", err);
                        None
                    }
                }
            }
            Err(err) => {
                eprintln!("{}", err);
                None
            }
        }
    }

    ///
    /// ポケモン 一覧
    ///
    fn list(&self) -> Option<Vec<Pokemon>> {
        match self.pool.get_conn() {
            Ok(mut conn) => {
                let query = "
                SELECT 
                    pokemon.pokemon_id, 
                    pokemon.number, 
                    pokemon.name,
                    pokemon.english_name,
                    pokemon.type1_id,
                    type1.name AS type1_name,
                    pokemon.type2_id,
                    type2.name AS type2_name,
                    pokemon.generation_id,
                    generation.value AS generation_value,
                    pokemon.hp,
                    pokemon.attack,
                    pokemon.defense,
                    pokemon.sp_attack,
                    pokemon.sp_defense,
                    pokemon.speed
                FROM 
                    pokemon
                    INNER JOIN type AS type1 ON type1.type_id = pokemon.type1_id
                    LEFT JOIN type AS type2 ON type2.type_id = pokemon.type2_id
                    INNER JOIN generation ON generation.generation_id = pokemon.generation_id

                ORDER BY
                    pokemon.number
                ";
                match conn.exec::<PokemonSchema, _, _>(query, ()) {
                    Ok(res) => Some(res.iter().map(|schema| schema.to_model()).collect()),
                    Err(err) => {
                        eprintln!("{}", err);
                        None
                    }
                }
            }
            Err(err) => {
                eprintln!("{}", err);
                None
            }
        }
    }
}
