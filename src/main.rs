use axum::Router;
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

use crate::routes::posts;

pub mod routes;
pub mod types;

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    // TODO: Do following with config files or smth
    let is_devenv = true;

    let socket_location = if is_devenv {
        "/home/cowe/repos/cowedev-backend/.tmp/"
    } else {
        "/var/lib/postgresql/"
    };
    let database_name = "cowedev-blogposts";

    // TODO: Proper error handling
    // TODO: Options file or smth
    let opts = PgConnectOptions::new()
        .socket(socket_location)
        .database(database_name);

    let pool = PgPoolOptions::new()
        .max_connections(5) // <- Seems low (in theory anyway)
        .connect_with(opts)
        .await
        .unwrap();

    let state = AppState { pool };

    let router = Router::new().merge(posts::router()).with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
