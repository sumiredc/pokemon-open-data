use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn ok() -> impl Responder {
    HttpResponse::Ok()
}
