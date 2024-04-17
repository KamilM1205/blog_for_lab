use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::DbError, models::Blog};

pub async fn init_db(client: &Client) -> Result<(), DbError> {
    let stmt = include_str!("../sql/schema.sql");

  	client.batch_execute(&stmt).await?;

    Ok(())
}

pub async fn get_blogs(client: &Client) -> Result<Vec<Blog>, DbError> {
    let stmt = include_str!("../sql/get_blogs.sql");
    let stmt = stmt.replace("$table_fields", &Blog::sql_table_fields());
    let stmt = client.prepare(&stmt).await?;

    let result = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Blog::from_row_ref(row).unwrap())
        .collect::<Vec<Blog>>();

    Ok(result)
}

pub async fn add_blog(client: &Client, blog: Blog) -> Result<Blog, DbError> {
    let stmt = include_str!("../sql/add_blog.sql");
    let stmt = stmt.replace("$table_fields", &Blog::sql_table_fields());
    let stmt = client.prepare(&stmt).await?;

    client
        .query(&stmt, &[&blog.title, &blog.description])
        .await?
        .iter()
        .map(|row| Blog::from_row_ref(row).unwrap())
        .collect::<Vec<Blog>>()
        .pop()
        .ok_or(DbError::NotFound)
}
