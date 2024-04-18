use actix_web::{web, Error, HttpResponse, ResponseError};
use chrono::Utc;

use crate::{auth_handler::LoggedUser, config::ServerConfig, db, errors::{DbError, ServiceError}, models::{Article, Articles}};

pub async fn add_article(
    article: web::Json<Article>,
    data: web::Data<ServerConfig>,
    user: Option<LoggedUser>,
) -> Result<HttpResponse, Error> {
	let user = user.ok_or(ServiceError::Unauthorized)?;

    let mut article = article.into_inner();

    article.author_id = user.id;
    article.date = Utc::now().naive_utc();

    let client = data.pg.get().await.map_err(DbError::PoolError)?;

    let new_article = db::add_article(&client, article).await?;

    let articles = Articles {id: 0, blog_id: new_article.blog_id, article_id: new_article.id};
    db::add_articles(&client, articles).await?;

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
    user: Option<LoggedUser>,
) -> Result<HttpResponse, Error> {
	let user = user.ok_or(ServiceError::Unauthorized)?;

    let id = id.into_inner();
    let client = data.pg.get().await.map_err(DbError::PoolError)?;

    let article = db::get_article(&client, id).await?;

    if article.author_id != user.id {
    	return Ok(ServiceError::BadRequest("You are not a onwer of this article.".into()).error_response());
    }

    db::delete_articles(&client, id).await?;
    let count = db::delete_article(&client, id).await?;

    Ok(HttpResponse::Ok().body(count.to_string()))
}
