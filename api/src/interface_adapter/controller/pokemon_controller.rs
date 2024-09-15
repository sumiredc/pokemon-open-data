use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};

use crate::{
    infrastructure::actix::router::RequestContext,
    interface_adapter::responder::{
        error_responder::ErrorResponder, pokemon_responder::PokemonReponder,
    },
    use_case::{
        input::pokemon_get_input::PokemonGetInput, pokemon_get_use_case::PokemonGetUseCase,
        pokemon_list_use_case::PokemonListUseCase,
    },
};

#[get("/pokemon")]
async fn index(data: Data<RequestContext>) -> impl Responder {
    let use_case = PokemonListUseCase::new(data.pokemon_repository());

    match use_case.execute() {
        Ok(output) => {
            let responder = PokemonReponder {};
            HttpResponse::Ok().json(responder.create_all(output.data()))
        }
        Err(_) => {
            let responder = ErrorResponder {};
            HttpResponse::InternalServerError().json(responder.internal_server_error())
        }
    }
}

#[get("/pokemon/{number}")]
async fn show(data: Data<RequestContext>, number: Path<u16>) -> impl Responder {
    let use_case = PokemonGetUseCase::new(data.pokemon_repository());
    let input = PokemonGetInput::new(number.to_owned());

    match use_case.execute(input) {
        Ok(output) => {
            let responder = PokemonReponder {};
            HttpResponse::Ok().json(responder.create(output.data()))
        }
        Err(_) => {
            let responder = ErrorResponder {};
            HttpResponse::InternalServerError().json(responder.internal_server_error())
        }
    }
}
