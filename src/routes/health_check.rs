use actix_web::HttpResponse;

pub async fn health_ckeck() -> HttpResponse {
    HttpResponse::Ok().finish()
}
