use actix_web::{HttpRequest, HttpResponse};

pub fn get_goods(req: HttpRequest) -> HttpResponse {
    // send to api method
    HttpResponse::Ok().finish()
}

pub fn get_good_delailed(req: HttpRequest) -> HttpResponse {
    // send to api method
    HttpResponse::Ok().finish()
}
