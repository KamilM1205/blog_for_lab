use actix_web::{web, Error, HttpResponse};

use crate::{config::ServerConfig, db, errors::DbError, models::Articles};

pub async fn get_articles(
    id: web::Path<i64>,
    data: web::Data<ServerConfig>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let client = data.pg.get().await.map_err(DbError::PoolError)?;

    let article = db::get_articles(&client, id).await?;

    Ok(HttpResponse::Ok().json(article))
}
