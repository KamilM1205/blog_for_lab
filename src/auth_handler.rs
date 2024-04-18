use std::future::{ready, Ready};

use actix_identity::Identity;
use actix_web::{web, Error, FromRequest, HttpMessage, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::{
    config::ServerConfig,
    db,
    errors::{DbError, ServiceError},
    models::Author,
    utils::verify_password,
};

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

pub type LoggedUser = Author;

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Ready<Result<LoggedUser, Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, payload).into_inner() {
            if let Ok(user_json) = identity.id() {
                if let Ok(user) = serde_json::from_str(&user_json) {
                    return ready(Ok(user));
                }
            }
        }

        ready(Err(ServiceError::Unauthorized.into()))
    }
}

pub async fn logout(id: Identity) -> HttpResponse {
    id.logout();
    HttpResponse::NoContent().finish()
}

pub async fn login(
    req: HttpRequest,
    auth_data: web::Json<AuthData>,
    data: web::Data<ServerConfig>,
) -> Result<HttpResponse, Error> {
    let client = data.pg.get().await.map_err(DbError::PoolError)?;
    let author = db::get_author_by_email(&client, &auth_data.email).await?;

    if let Ok(matching) = verify_password(&author.password, &auth_data.password) {
        if matching {
            let user_str = serde_json::to_string(&author)?;
            Identity::login(&req.extensions(), user_str)?;
        }
    }

    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_me(logged_user: LoggedUser) -> HttpResponse {
    HttpResponse::Ok().json(logged_user)
}
