use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "\"testing.Author")]
pub struct Author {
    pub id: i64,
    pub nickname: String,
    pub name: String,
    pub surname: String,
    pub date: NaiveDateTime,
    pub email: String,
    pub phone: String,
    pub image: String,
    pub birthday: NaiveDateTime,
    pub password: String,
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "\"testing.Article\"")]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub file_name: String,
    pub date: NaiveDateTime,
    pub author_id: i64,
    pub blog_id: i64,
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "\"testing.Articles\"")]
pub struct Articles {
    pub id: i64,
    pub blog_id: i64,
    pub article_id: i64,
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "\"testing.Blog\"")]
pub struct Blog {
	#[serde(skip_deserializing)]
    pub id: i64,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "\"testing.Category\"")]
pub struct Category {
    pub id: i64,
    pub blog_id: i64,
    pub name: String,
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "\"testing.Category_Type\"")]
pub struct CategoryType {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "\"testing.Comment\"")]
pub struct Comment {
    pub id: i64,
    pub article_id: i64,
    pub author_id: i64,
    pub text: String,
}
