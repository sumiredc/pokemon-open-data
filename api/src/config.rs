use std::env;

use dotenv::dotenv;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Config {
    pub client_url: String,
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        Self {
            client_url: env::var("CLIENT_URL").unwrap_or_default(),
            database_url: env::var("DATABASE_URL").unwrap_or_default(),
            host: env::var("HOST").unwrap_or_default(),
            port: env::var("PORT")
                .unwrap_or_default()
                .parse::<u16>()
                .unwrap_or_default(),
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv().ok();
    Config::new()
});
