use actix_swagger_derive::swagger;
use actix_web::{Responder, HttpResponse, FromRequest};
use std::future::Future;

trait FromRequestSealed<F> {}

impl<T, F> FromRequestSealed<F> for F
where T: FromRequest,
      T::Future: Future<Output = Result<F, T::Error>>
{}

struct TestStruct;

#[swagger]
async fn list_pets<R1>(req: R1) -> impl Responder
where
    R1: FromRequestSealed<TestStruct>
{
    req
}
