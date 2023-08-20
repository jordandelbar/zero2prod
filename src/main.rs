use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn health_ckeck() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_ckeck)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
