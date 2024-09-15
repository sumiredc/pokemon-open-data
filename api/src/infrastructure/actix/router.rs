use actix_web::{web::Data, App, HttpServer};
use mysql::Pool;

use crate::{
    config::CONFIG, domain::model::repository::PokemonRepository,
    infrastructure::mysql::pokemon_repository::PokemonRepositoryImpl, interface_adapter,
};

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    let addr = format!("{}:{}", CONFIG.host, CONFIG.port);

    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(RequestContext::new()).clone())
            .service(interface_adapter::controller::pokemon_controller::index)
            .service(interface_adapter::controller::pokemon_controller::show)
            .service(interface_adapter::controller::dashboard::index)
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
}
