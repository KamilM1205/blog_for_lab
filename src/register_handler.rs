use actix_web::{web, Error, HttpResponse};

use crate::{config::ServerConfig, models::Author};


pub async fn register_author(
	author: web::Json<Author>,
	data: ServerConfig,
) -> Result<HttpResponse, Error> {
	let connection = data.pg.get().await?;

	let author = author.into_inner();
	author.password = sha256::digest(author.password);

	
}
