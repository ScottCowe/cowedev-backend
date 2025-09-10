use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BlogpostData {
    pub id: String,
    pub title: String,
    pub format: String,
    pub created_on: NaiveDateTime,
    pub updated_on: Option<NaiveDateTime>,
    pub tags: Option<Vec<String>>,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Blogpost {
    pub id: String,
    pub title: String,
    pub format: String,
    pub created_on: NaiveDateTime,
    pub updated_on: Option<NaiveDateTime>,
    pub tags: Option<Vec<String>>,
    pub content: String,
}
