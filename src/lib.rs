use actix_web::{web, App, HttpResponse, HttpServer};

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_ckeck)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

async fn health_ckeck() -> HttpResponse {
    HttpResponse::Ok().finish()
}
