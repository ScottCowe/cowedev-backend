use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BlogpostData {
    pub id: String,
    pub title: String,
    pub format: String,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Blogpost {
    pub id: String,
    pub title: String,
    pub format: String,
    pub content: String,
}
