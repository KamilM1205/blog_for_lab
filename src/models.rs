use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "\"testing.Author\"")]
pub struct Author {
    #[serde(default)]
    pub id: i64,
    pub nickname: String,
    pub name: String,
    pub surname: String,
    #[serde(skip_deserializing)]
    pub date: NaiveDate,
    pub email: String,
    pub phone: String,
    #[serde(skip_deserializing)]
    pub image: String,
    pub birthday: NaiveDate,
    pub password: String,
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "\"testing.Article\"")]
pub struct Article {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub title: String,
    #[serde(skip_deserializing)]
    pub file_name: String,
    #[serde(skip_deserializing)]
    pub date: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub author_id: i64,
    pub blog_id: i64,
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "\"testing.Articles\"")]
pub struct Articles {
    #[serde(skip_deserializing)]
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
    #[serde(skip_deserializing)]
    pub id: i64,
    pub blog_id: i64,
    pub name: String,
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "\"testing.Category_Type\"")]
pub struct CategoryType {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "\"testing.Comment\"")]
pub struct Comment {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub article_id: i64,
    #[serde(skip_deserializing)]
    pub author_id: i64,
    pub text: String,
}
