use axum::{Router, routing::get, Json};
use serde::{Serialize, Deserialize};
use tokio_postgres::NoTls;
use deadpool_postgres::{Manager, Pool};
use dotenv::dotenv;
use std::env;

// Serde-Datenstruktur für die Antwort
#[derive(Serialize, Deserialize)]
struct Item {
    id: i32,
    name: String,
}

// Funktion zum Abrufen von Items aus der PostgreSQL-Datenbank
async fn fetch_items(pool: &Pool) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    // Hole einen Client aus dem Pool
    let client = pool.get().await?;
    
    // Führe eine SQL-Abfrage aus
    let rows = client.query("SELECT id, email FROM users", &[]).await?;

    // Verarbeite die Ergebnisse und gebe eine Liste von Items zurück
    let items: Vec<Item> = rows.iter().map(|row| Item {
        id: row.get(0),
        name: row.get(1),
    }).collect();

    Ok(items)
}

// API-Handler für den Endpunkt "/items"
async fn get_items_handler(pool: Pool) -> Json<Vec<Item>> {
    match fetch_items(&pool).await {
        Ok(items) => Json(items),
        Err(_) => Json(vec![]),  // Falls ein Fehler auftritt, eine leere Liste zurückgeben
    }
}

// Funktion zur Einrichtung des Deadpool-Pools
async fn create_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var_os("DATABASE_URL").expect("DATABASE_URL not set in .env");
    let manager = Manager::new(database_url, NoTls);
    Pool::new(manager, 4)  // 16 Verbindungen im Pool
}

#[tokio::main]
async fn main() {
    // Erstelle den Pool
    let pool = create_pool().await;

    // Definiere die Routen für Axum
    let app = Router::new().route("/items", get(move || get_items_handler(pool.clone())));

    // Starte den HTTP-Server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}