use mysql::{
    prelude::{FromRow, Queryable},
    Pool,
};

use crate::domain::model::{repository::SummaryRepository, summary::Summary};

pub struct SummaryRepositoryImpl {
    pub pool: Box<Pool>,
}

#[derive(FromRow)]
struct SummarySchema {
    label: String,
    value: usize,
}

impl SummarySchema {
    pub fn to_model(&self) -> Summary {
        Summary::new(self.label.clone(), self.value)
    }
}

impl SummaryRepository for SummaryRepositoryImpl {
    ///
    /// タイプ 集計
    ///
    fn r#type(&self) -> Option<Vec<Summary>> {
        match self.pool.get_conn() {
            Ok(mut conn) => {
                let query = "
                SELECT
                    name AS label,
                    count(*) AS value
                FROM
                    (
                        SELECT
                            type1_id AS type_id
                        FROM
                            pokemon
                        UNION ALL
                        SELECT
                            type2_id AS type_id
                        FROM
                            pokemon
                        WHERE
                            type2_id IS NOT NULL
                    ) AS typeall
                    INNER JOIN type ON type.type_id = typeall.type_id
                GROUP BY
                    label
                ";

                match conn.exec::<SummarySchema, _, _>(query, ()) {
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

    ///
    /// 世代 集計
    ///
    fn generation(&self) -> Option<Vec<Summary>> {
        match self.pool.get_conn() {
            Ok(mut conn) => {
                let query = "
                SELECT
                    CONVERT(generation.value, CHAR) AS label,
                    count(*) AS value
                FROM
                    pokemon
                    INNER JOIN generation ON generation.generation_id = pokemon.generation_id
                GROUP BY
                    label
                ";

                match conn.exec::<SummarySchema, _, _>(query, ()) {
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

    ///
    /// ステータス 集計
    ///
    fn stats(&self) -> Option<Vec<Summary>> {
        match self.pool.get_conn() {
            Ok(mut conn) => {
                let query = "
                SELECT
                    CASE 
                        WHEN total <= 200 THEN '~200'
                        WHEN total BETWEEN 201 AND 300 THEN '201~300'
                        WHEN total BETWEEN 301 AND 400 THEN '301~400'
                        WHEN total BETWEEN 401 AND 500 THEN '401~500'
                        WHEN total BETWEEN 501 AND 600 THEN '501~600'
                        WHEN total BETWEEN 601 AND 700 THEN '601~700'
                        ELSE '701~'
                    END AS label,
                    COUNT(*) AS value
                FROM
                    (
                        SELECT
                            (
                                hp + attack + defense + sp_attack + sp_defense + speed
                            ) AS total
                        FROM
                            pokemon
                    ) AS stats
                GROUP BY label
                ";

                match conn.exec::<SummarySchema, _, _>(query, ()) {
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
