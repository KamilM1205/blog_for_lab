use actix_web::{web, Error, HttpResponse, ResponseError};

use crate::{
    config::ServerConfig,
    db::{self, add_author, get_author_by_email, get_author_by_nickname},
    errors::{DbError, ServiceError},
    models::Author,
    utils::hash_password,
};

pub async fn register_author(
    author: web::Json<Author>,
    data: web::Data<ServerConfig>,
) -> Result<HttpResponse, Error> {
    let client = data.pg.get().await.map_err(DbError::PoolError)?;

    let mut author = author.into_inner();

    if author.password.len() < 6 {
        return Ok(
            ServiceError::BadRequest("Password length must be 6 or more.".into()).error_response(),
        );
    }

    if author.nickname.len() < 4 {
        return Ok(
            ServiceError::BadRequest("Nickname length must be 4 or more.".into()).error_response(),
        );
    }

    if author.name.len() == 0 || author.surname.len() == 0 || author.phone.len() == 0 {
        return Ok(ServiceError::BadRequest("Some fileds is empty".into()).error_response());
    }

    author.password = hash_password(&author.password)?;

    if get_author_by_email(&client, &author.email).await.is_ok() {
        return Ok(ServiceError::BadRequest("Email already used.".into()).error_response());
    }

    if get_author_by_nickname(&client, &author.nickname)
        .await
        .is_ok()
    {
        return Ok(ServiceError::BadRequest("Nickname already used.".into()).error_response());
    }

    let new_author = add_author(&client, author).await?;

    Ok(HttpResponse::Ok().json(&new_author))
}
