use actix_web::{web, Error, HttpResponse};

use crate::{config::ServerConfig, db, errors::DbError, models::Blog};

pub async fn add_blog(
    blog: web::Json<Blog>,
    data: web::Data<ServerConfig>,
) -> Result<HttpResponse, Error> {
    let blog = blog.into_inner();

    let client = data.pg.get().await.map_err(DbError::PoolError)?;

    let new_blog = db::add_blog(&client, blog).await?;

    Ok(HttpResponse::Ok().json(new_blog))
}

pub async fn get_blogs(data: web::Data<ServerConfig>) -> Result<HttpResponse, Error> {
    let client = data.pg.get().await.map_err(DbError::PoolError)?;

    let blogs = db::get_blogs(&client).await?;

    Ok(HttpResponse::Ok().json(blogs))
}
