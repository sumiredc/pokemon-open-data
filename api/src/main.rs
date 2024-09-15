mod config;
mod domain;
mod infrastructure;
mod interface_adapter;
mod use_case;

use infrastructure::actix::router;

fn main() -> std::io::Result<()> {
    router::run()
}
