mod handler;


use axum::response::IntoResponse;
use tokio_postgres::{NoTls, Row};
use tokio_postgres::Error;
use axum::{ Router};
use axum::routing::get;

#[derive(Debug)]
pub struct User {
  pub id: i32,
  pub email: String,
}

impl From<Row> for User {
  fn from(row: Row) -> Self {
    Self {
      id: row.get("id"),
      email: row.get("email"),
    }
  }
}

// Using Tokio runtime for async behaviour: https://crates.io/crates/tokio
#[tokio::main]
async fn main() -> Result<(), Error> {

    let app = Router::new()
    .route("/", get(|| async { Ok::<_, axum::http::StatusCode>("Hello, World!") }));
  
 //   async fn get_users_handler() -> impl IntoResponse {
       
   // }
  // Get DB client and connection
  let (client, connection) = tokio_postgres::connect(
    "postgres://postgres:23hermann75%40#*@127.0.0.1:5432/nails?sslmode=disable",
    NoTls,
  )
  .await?;

  // Spawn connection
  tokio::spawn(async move {
    if let Err(error) = connection.await {
      eprintln!("Connection error: {}", error);
    }
  });

  // Do the query
  let result = client.query("SELECT id, email FROM users", &[]).await?;

  let users: Vec<User> = result.into_iter().map(|row| User::from(row)).collect();

  println!("{:#?}", users);

  axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
  Ok(())
}