use actix_web::{web, HttpResponse};

use crate::handlers::api::*;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(
                web::scope("/probe")
                    .service(web::resource("/liveness").route(web::get().to(|| HttpResponse::Ok())))
                    .service(
                        web::resource("/readiness").route(web::get().to(|| HttpResponse::Ok())),
                    )
                    .service(web::resource("/startup").route(web::get().to(|| HttpResponse::Ok()))),
            )
            .service(
                web::scope("/goods")
                    .service(web::resource("").route(web::get().to(get_goods)))
                    .service(web::resource("/{id}").route(web::get().to(get_good_detailed))),
            ),
    );
}
