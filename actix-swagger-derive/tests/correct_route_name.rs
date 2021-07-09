use actix_swagger_derive::swagger;
use actix_web::{Responder, HttpResponse};

#[swagger]
async fn list_pets() -> impl Responder {
    HttpResponse::Ok()
}
