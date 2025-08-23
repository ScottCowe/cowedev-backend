use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use serde_json::{Value, json};

use crate::AppState;
use crate::types::{Blogpost, BlogpostData};

pub async fn get_posts(State(state): State<AppState>) -> Json<Value> {
    let posts: Vec<BlogpostData> = sqlx::query_as(r#"SELECT id, title, format FROM posts"#)
        .fetch_all(&state.pool)
        .await
        .unwrap();

    Json(json!(posts))
}

pub async fn get_post(State(state): State<AppState>, Path(path): Path<String>) -> Json<Value> {
    // TODO: Make sure this is actually safe
    let post: Vec<Blogpost> = sqlx::query_as(r#"SELECT * FROM posts WHERE id=$1"#)
        .bind(path)
        .fetch_all(&state.pool)
        .await
        .unwrap();

    if post.is_empty() {
        // TODO: Make this properly fail and stuff
        return Json(json!({}));
    }

    Json(json!(post[0]))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/posts", get(get_posts))
        .route("/api/posts/{id}", get(get_post))
}
