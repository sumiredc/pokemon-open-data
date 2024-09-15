use mysql::{
    params,
    prelude::{FromRow, Queryable},
    FromRowError, Pool, Row,
};

use crate::domain::model::{pokemon::Pokemon, repository::PokemonRepository};

pub struct PokemonRepositoryImpl {
    pub pool: Box<Pool>,
}

impl FromRow for Pokemon {
    fn from_row_opt(row: Row) -> Result<Pokemon, FromRowError> {
        mysql::from_row_opt(row).map(|(number, name)| Pokemon::new(number, name))
    }
}

impl PokemonRepository for PokemonRepositoryImpl {
    fn find_by_number(&self, number: u16) -> Option<Pokemon> {
        match self.pool.get_conn() {
            Ok(mut conn) => {
                return conn
                    .exec_first::<Pokemon, _, _>(
                        r#"SELECT number, name FROM pokemon WHERE number = :number"#,
                        params! {
                            "number" => number
                        },
                    )
                    .unwrap_or_default()
            }
            Err(_) => return None,
        }
    }
    fn list(&self) -> Option<Vec<Pokemon>> {
        match self.pool.get_conn() {
            Ok(mut conn) => {
                return conn
                    .exec::<Pokemon, _, _>("SELECT number, name FROM pokemon", ())
                    .map(|res| Some(res))
                    .unwrap_or_default()
            }
            Err(_) => return None,
        }
    }
}
