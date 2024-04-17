use actix_web::{web, Error, HttpResponse};

use crate::{config::ServerConfig, db, errors::DbError, models::Author};

pub async fn add_author(
    author: web::Json<Author>,
    data: web::Data<ServerConfig>,
) -> Result<HttpResponse, Error> {
    let author = author.into_inner();

    let client = data.pg.get().await.map_err(DbError::PoolError)?;

    let new_author = db::add_author(&client, author).await?;

    Ok(HttpResponse::Ok().json(new_author))
}

pub async fn get_author(id: web::Path<i64>, data: web::Data<ServerConfig>) -> Result<HttpResponse, Error> {
	let id = id.into_inner();
    let client = data.pg.get().await.map_err(DbError::PoolError)?;

    let author = db::get_author(&client, id).await?;

    Ok(HttpResponse::Ok().json(author))
}