use actix_cors::Cors;
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use mysql::Pool;

use crate::{
    config::CONFIG,
    domain::model::repository::{PokemonRepository, SummaryRepository},
    infrastructure::mysql::{
        pokemon_repository::PokemonRepositoryImpl, summary_repository::SummaryRepositoryImpl,
    },
    interface_adapter,
};

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    let addr = format!("{}:{}", CONFIG.host, CONFIG.port);
    let client_url = CONFIG.client_url.as_str();

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(client_url)
                    .allowed_methods(vec!["GET"])
                    .max_age(3600),
            )
            .app_data(Data::new(RequestContext::new()).clone())
            .service(interface_adapter::controller::check_controller::ok)
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .service(interface_adapter::controller::pokemon_controller::index)
                        .service(interface_adapter::controller::pokemon_controller::show)
                        .service(interface_adapter::controller::summary_controller::r#type)
                        .service(interface_adapter::controller::summary_controller::generation)
                        .service(interface_adapter::controller::summary_controller::stats),
                ),
            )
    })
    .bind(addr)?
    .run()
    .await
}

#[derive(Clone)]
pub struct RequestContext {
    pub pool: Pool,
}

impl RequestContext {
    pub fn new() -> RequestContext {
        let url = CONFIG.database_url.as_str();
        let pool = Pool::new(url).expect("Failed to create DB connection pool.");

        RequestContext { pool }
    }

    pub fn pokemon_repository(&self) -> impl PokemonRepository {
        PokemonRepositoryImpl {
            pool: Box::new(self.pool.to_owned()),
        }
    }

    pub fn summary_repository(&self) -> impl SummaryRepository {
        SummaryRepositoryImpl {
            pool: Box::new(self.pool.to_owned()),
        }
    }
}
