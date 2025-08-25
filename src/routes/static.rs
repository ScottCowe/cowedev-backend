use axum::Router;
use std::env;
use tower_http::services::ServeDir;

use crate::AppState;

pub fn router() -> Router<AppState> {
    let data_dir = env::var("DATA_DIR").expect("DATA_DIR envar not set");

    Router::new().nest_service("/static", ServeDir::new(data_dir))
}
