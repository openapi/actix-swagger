use actix_web::{get, web, App, HttpServer, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct IndexPath {
    id: u32,
    name: String,
}

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
