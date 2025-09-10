use std::{env, fs};

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use serde_json::{Value, json};

use crate::AppState;
use crate::types::{Blogpost, BlogpostData};

pub async fn get_posts(State(state): State<AppState>) -> Json<Value> {
    // TODO: Switch to using query_as macro
    let posts: Vec<BlogpostData> = sqlx::query_as(r#"SELECT * FROM posts"#)
        .fetch_all(&state.pool)
        .await
        .unwrap();

    Json(json!(posts))
}

pub async fn get_post(State(state): State<AppState>, Path(id): Path<String>) -> Json<Value> {
    // TODO: Make sure this is actually safe
    // TODO: Switch to using query_as macro
    let post_data: BlogpostData = sqlx::query_as(r#"SELECT * FROM posts WHERE id=$1"#)
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    let data_dir = env::var("DATA_DIR").expect("DATA_DIR envar not set");

    let path = match post_data.format.as_str() {
        "plaintext" => format!("{}/{}/content.txt", data_dir, post_data.id),
        "html" => format!("{}/{}/content.html", data_dir, post_data.id),
        _ => "".to_string(),
    };

    let content = fs::read_to_string(path).expect("File not reading");

    let post = Blogpost {
        id: post_data.id,
        title: post_data.title,
        format: post_data.format,
        created_on: post_data.created_on,
        updated_on: post_data.updated_on,
        tags: post_data.tags,
        content: content,
    };

    Json(json!(post))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/posts", get(get_posts))
        .route("/api/posts/{id}", get(get_post))
}
