use axum::{routing::get, Router};
use dotenv::dotenv;

use crate::routes::auth::auth_routes;

mod db;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .nest("/auth", auth_routes());

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Hello, world!");
}
