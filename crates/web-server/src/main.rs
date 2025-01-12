mod config;
mod errors;


use std::net::SocketAddr;

use axum::{routing::get, Extension, Router};
use tower_livereload::LiveReloadLayer;
use serde::{Serialize, Deserialize};

use tokio_postgres::{NoTls, Error};
#[derive(Serialize, Deserialize)]
struct Item {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    // build our application with a route
    let app = Router::new()
        .route("/", get(root::loader))
        .layer(LiveReloadLayer::new())
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}