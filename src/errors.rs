use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

#[derive(Display, From, Debug)]
pub enum DbError {
    NotFound,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
}

impl std::error::Error for DbError {}

impl ResponseError for DbError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match *self {
            DbError::NotFound => HttpResponse::NotFound().finish(),
            DbError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}

#[derive(Debug, Display, From)]
pub enum ServiceError {
    InternalServerError,
    Unauthorized,
    BadRequest(String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError()
                .json("Internal server error, please try again later"),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
        }
    }
}
