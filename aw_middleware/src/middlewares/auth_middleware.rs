use actix_web::{Error, FromRequest, Result, error::ErrorUnauthorized};
use futures::future::{Ready, ready};

pub struct AuthMiddleware {
    pub user_id: i32,
}

impl FromRequest for AuthMiddleware {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match req.cookie("user_id") {
            Some(c) => match c.value().parse::<i32>() {
                Ok(id) => ready(Ok(AuthMiddleware { user_id: id })),
                Err(_) => ready(Err(ErrorUnauthorized("Token"))),
            },
            None => ready(Err(ErrorUnauthorized("Token tidak valid"))),
        }
    }
}
