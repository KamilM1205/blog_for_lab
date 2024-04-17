use actix_web::{web, Error, HttpResponse};

use crate::{config::ServerConfig, db, errors::DbError, models::Comment};

pub async fn add_comment(
    comment: web::Json<Comment>,
    data: web::Data<ServerConfig>,
) -> Result<HttpResponse, Error> {
    let comment = comment.into_inner();

    let client = data.pg.get().await.map_err(DbError::PoolError)?;

    let new_article = db::add_comment(&client, comment).await?;

    Ok(HttpResponse::Ok().json(new_article))
}

pub async fn get_comments(id: web::Path<i64>, data: web::Data<ServerConfig>) -> Result<HttpResponse, Error> {
	let id = id.into_inner();
    let client = data.pg.get().await.map_err(DbError::PoolError)?;

    let comment = db::get_comments(&client, id).await?;

    Ok(HttpResponse::Ok().json(comment))
}

pub async fn delete_comment(id: web::Path<i64>, data: web::Data<ServerConfig>) -> Result<HttpResponse, Error> {
	let id = id.into_inner();
	let client = data.pg.get().await.map_err(DbError::PoolError)?;

	let count = db::delete_comment(&client, id).await?;

	Ok(HttpResponse::Ok().body(count.to_string()))
}