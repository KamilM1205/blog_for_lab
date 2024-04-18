use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
    errors::DbError,
    models::{Article, Articles, Author, Blog, Category, CategoryType, Comment},
};

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

pub async fn add_article(client: &Client, article: Article) -> Result<Article, DbError> {
    let stmt = include_str!("../sql/add_article.sql");
    let stmt = stmt.replace("$table_fields", &Article::sql_table_fields());
    let stmt = client.prepare(&stmt).await?;

    client
        .query(
            &stmt,
            &[
                &article.title,
                &article.file_name,
                &article.date,
                &article.author_id,
                &article.blog_id,
            ],
        )
        .await?
        .iter()
        .map(|row| Article::from_row_ref(row).unwrap())
        .collect::<Vec<Article>>()
        .pop()
        .ok_or(DbError::NotFound)
}

pub async fn add_author(client: &Client, author: Author) -> Result<Author, DbError> {
    let stmt = include_str!("../sql/add_author.sql");
    let stmt = stmt.replace("$table_fields", &Author::sql_table_fields());
    let stmt = client.prepare(&stmt).await?;

    client
        .query(
            &stmt,
            &[
                &author.nickname,
                &author.name,
                &author.surname,
                &author.date,
                &author.email,
                &author.phone,
                &author.image,
                &author.birthday,
                &author.password,
            ],
        )
        .await?
        .iter()
        .map(|row| Author::from_row_ref(row).unwrap())
        .collect::<Vec<Author>>()
        .pop()
        .ok_or(DbError::NotFound)
}

pub async fn add_comment(client: &Client, comment: Comment) -> Result<Comment, DbError> {
    let stmt = include_str!("../sql/add_comment.sql");
    let stmt = stmt.replace("$table_fields", &Comment::sql_table_fields());
    let stmt = client.prepare(&stmt).await?;

    client
        .query(
            &stmt,
            &[&comment.article_id, &comment.author_id, &comment.text],
        )
        .await?
        .iter()
        .map(|row| Comment::from_row_ref(row).unwrap())
        .collect::<Vec<Comment>>()
        .pop()
        .ok_or(DbError::NotFound)
}

pub async fn delete_article(client: &Client, article_id: i64) -> Result<u64, DbError> {
    let stmt = include_str!("../sql/delete_article.sql");
    let stmt = client.prepare(&stmt).await?;

    Ok(client.execute(&stmt, &[&article_id]).await?)
}

pub async fn delete_comment(client: &Client, comment_id: i64) -> Result<u64, DbError> {
    let stmt = include_str!("../sql/delete_comment.sql");
    let stmt = client.prepare(&stmt).await?;

    Ok(client.execute(&stmt, &[&comment_id]).await?)
}

pub async fn get_article(client: &Client, article_id: i64) -> Result<Article, DbError> {
    let stmt = include_str!("../sql/get_article.sql");
    let stmt = stmt.replace("$table_fields", &Article::sql_table_fields());
    let stmt = client.prepare(&stmt).await?;

    let result = client.query_one(&stmt, &[&article_id]).await?;

    let result = Article::from_row(result)?;

    Ok(result)
}

pub async fn get_articles(client: &Client, blog_id: i64) -> Result<Vec<Articles>, DbError> {
    let stmt = include_str!("../sql/get_articles.sql");
    let stmt = stmt.replace("$table_fields", &Articles::sql_table_fields());
    let stmt = client.prepare(&stmt).await?;

    let result = client
        .query(&stmt, &[&blog_id])
        .await?
        .iter()
        .map(|row| Articles::from_row_ref(row).unwrap())
        .collect::<Vec<Articles>>();

    Ok(result)
}

pub async fn get_author(client: &Client, author_id: i64) -> Result<Author, DbError> {
    let stmt = include_str!("../sql/get_author.sql");
    let stmt = stmt.replace("$table_fields", &Author::sql_table_fields());
    let stmt = client.prepare(&stmt).await?;

    let result = client.query_one(&stmt, &[&author_id]).await?;

    let result = Author::from_row(result)?;

    Ok(result)
}

pub async fn get_author_by_email(client: &Client, email: &str) -> Result<Author, DbError> {
    let stmt = include_str!("../sql/get_author_by_email.sql");
    let stmt = stmt.replace("$table_fields", &Author::sql_table_fields());
    let stmt = client.prepare(&stmt).await?;

    let result = client.query_one(&stmt, &[&email]).await?;
    let result = Author::from_row(result)?;

    Ok(result)
}

pub async fn get_author_by_nickname(client: &Client, nickname: &str) -> Result<Author, DbError> {
    let stmt = include_str!("../sql/get_author_by_nickname.sql");
    let stmt = stmt.replace("$table_fields", &Author::sql_table_fields());
    let stmt = client.prepare(&stmt).await?;

    let result = client.query_one(&stmt, &[&nickname]).await?;
    let result = Author::from_row(result)?;

    Ok(result)
}

pub async fn get_category(client: &Client) -> Result<Vec<Category>, DbError> {
    let stmt = include_str!("../sql/get_category.sql");
    let stmt = stmt.replace("$table_fields", &Category::sql_table_fields());
    let stmt = client.prepare(&stmt).await?;

    let result = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Category::from_row_ref(row).unwrap())
        .collect::<Vec<Category>>();

    Ok(result)
}

pub async fn get_category_types(client: &Client) -> Result<Vec<CategoryType>, DbError> {
    let stmt = include_str!("../sql/get_category_types.sql");
    let stmt = stmt.replace("$table_fields", &CategoryType::sql_table_fields());
    let stmt = client.prepare(&stmt).await?;

    let result = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| CategoryType::from_row_ref(row).unwrap())
        .collect::<Vec<CategoryType>>();

    Ok(result)
}

pub async fn get_comments(client: &Client, article_id: i64) -> Result<Vec<Comment>, DbError> {
    let stmt = include_str!("../sql/get_comments.sql");
    let stmt = stmt.replace("$table_fields", &Comment::sql_table_fields());
    let stmt = client.prepare(&stmt).await?;

    let result = client
        .query(&stmt, &[&article_id])
        .await?
        .iter()
        .map(|row| Comment::from_row_ref(row).unwrap())
        .collect::<Vec<Comment>>();

    Ok(result)
}
