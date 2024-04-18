use actix_web::{web, Error, HttpResponse, ResponseError};

use crate::{auth_handler::LoggedUser, config::ServerConfig, db, errors::{DbError, ServiceError}, models::Comment};

pub async fn add_comment(
    comment: web::Json<Comment>,
    data: web::Data<ServerConfig>,
    user: Option<LoggedUser>,
) -> Result<HttpResponse, Error> {
    let user = user.ok_or(ServiceError::Unauthorized)?;
    let mut comment = comment.into_inner();

    comment.author_id = user.id;

    let client = data.pg.get().await.map_err(DbError::PoolError)?;

    let new_article = db::add_comment(&client, comment).await?;

    Ok(HttpResponse::Ok().json(new_article))
}

pub async fn get_comments(
    id: web::Path<i64>,
    data: web::Data<ServerConfig>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let client = data.pg.get().await.map_err(DbError::PoolError)?;

    let comment = db::get_comments(&client, id).await?;

    Ok(HttpResponse::Ok().json(comment))
}

pub async fn delete_comment(
    id: web::Path<i64>,
    data: web::Data<ServerConfig>,
    user: Option<LoggedUser>,
) -> Result<HttpResponse, Error> {
    let user = user.ok_or(ServiceError::Unauthorized)?;
    let id = id.into_inner();
    let client = data.pg.get().await.map_err(DbError::PoolError)?;

    let comment = db::get_comment(&client, id).await?;

    if user.id != comment.author_id {
        return Ok(ServiceError::BadRequest("You are not owner of this comment.".into()).error_response());
    }

    let count = db::delete_comment(&client, id).await?;

    Ok(HttpResponse::Ok().body(count.to_string()))
}
