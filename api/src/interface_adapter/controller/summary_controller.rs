use actix_web::{get, web::Data, HttpResponse, Responder};

use crate::{
    infrastructure::actix::router::RequestContext,
    interface_adapter::responder::{
        error_responder::ErrorResponder, summary_responder::SummaryResponder,
    },
    use_case::{
        summary_generation_use_case::SummaryGenerationUseCase,
        summary_stats_use_case::SummaryStatsUseCase, summary_type_use_case::SummaryTypeUseCase,
    },
};

#[get("/summary/type")]
async fn r#type(data: Data<RequestContext>) -> impl Responder {
    let use_case = SummaryTypeUseCase::new(data.summary_repository());

    match use_case.execute() {
        Ok(output) => {
            let responder = SummaryResponder {};
            HttpResponse::Ok().json(responder.create_all(output.data()))
        }
        Err(_) => {
            let responder = ErrorResponder {};
            HttpResponse::InternalServerError().json(responder.internal_server_error())
        }
    }
}

#[get("/summary/generation")]
async fn generation(data: Data<RequestContext>) -> impl Responder {
    let use_case = SummaryGenerationUseCase::new(data.summary_repository());

    match use_case.execute() {
        Ok(output) => {
            let responder = SummaryResponder {};
            HttpResponse::Ok().json(responder.create_all(output.data()))
        }
        Err(_) => {
            let responder = ErrorResponder {};
            HttpResponse::InternalServerError().json(responder.internal_server_error())
        }
    }
}

#[get("/summary/stats")]
async fn stats(data: Data<RequestContext>) -> impl Responder {
    let use_case = SummaryStatsUseCase::new(data.summary_repository());

    match use_case.execute() {
        Ok(output) => {
            let responder = SummaryResponder {};
            HttpResponse::Ok().json(responder.create_all(output.data()))
        }
        Err(_) => {
            let responder = ErrorResponder {};
            HttpResponse::InternalServerError().json(responder.internal_server_error())
        }
    }
}
