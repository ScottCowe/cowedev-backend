use std::env;

use axum::Router;
use dotenv::dotenv;
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

use crate::routes::{api, r#static};

pub mod routes;
pub mod types;

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let socket_path = env::var("SOCKET_PATH").expect("SOCKET_PATH envar not set");
    let database_name = env::var("DB_NAME").expect("DB_NAME envar not found");

    // TODO: Proper error handling
    let opts = PgConnectOptions::new()
        .socket(socket_path)
        .database(database_name);

    let pool = PgPoolOptions::new()
        .max_connections(5) // <- Seems low (in theory anyway)
        .connect_with(opts)
        .await
        .unwrap();

    let state = AppState { pool };

    let router = Router::new()
        .merge(api::router())
        .merge(r#static::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
