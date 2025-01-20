use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/api/artists", get(get_artists));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server läuft unter http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root_handler() -> &'static str {
    "Willkommen zur Online-Galerie!"
}

async fn get_artists() -> &'static str {
    "Liste der Künstler (in Arbeit)"
}