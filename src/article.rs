use actix_web::{web, Error, HttpResponse};

use crate::{config::ServerConfig, db, errors::DbError, models::Article};

pub async fn add_article(
    article: web::Json<Article>,
    data: web::Data<ServerConfig>,
) -> Result<HttpResponse, Error> {
    let article = article.into_inner();

    let client = data.pg.get().await.map_err(DbError::PoolError)?;

    let new_article = db::add_article(&client, article).await?;

    Ok(HttpResponse::Ok().json(new_article))
}

pub async fn get_article(
    id: web::Path<i64>,
    data: web::Data<ServerConfig>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let client = data.pg.get().await.map_err(DbError::PoolError)?;

    let article = db::get_article(&client, id).await?;

    Ok(HttpResponse::Ok().json(article))
}

pub async fn delete_article(
    id: web::Path<i64>,
    data: web::Data<ServerConfig>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let client = data.pg.get().await.map_err(DbError::PoolError)?;

    let count = db::delete_article(&client, id).await?;

    Ok(HttpResponse::Ok().body(count.to_string()))
}
