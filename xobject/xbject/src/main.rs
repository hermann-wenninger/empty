mod handler;


//use axum::response::IntoResponse;
use tokio_postgres::{NoTls, Row};
use tokio_postgres::Error;
use axum::{ Router};
use axum::routing::get;

#[derive(Debug,PartialEq)]
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
  let users = get_users().await?;

  println!("{:#?}", users);

  axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
  Ok(())
}

async fn get_users() -> Result<Vec<User>, Error> {
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

  Ok(users)
}
    #[tokio::test]
    async fn test_success() {
        let users = get_users().await.unwrap();
        assert_eq!(users,[
          User {
              id: 1,
              email: "test1@test1.com".to_string(),
          },
          User {
              id: 2,
              email: "test2@test1.com".to_string(),
          },
          User {
              id: 3,
              email: "test3@test1.com".to_string(),
          },
          User {
              id: 4,
              email: "test4@test1.com".to_string(),
          },
          User {
              id: 5,
              email: "test5@test1.com".to_string(),
          },
          User {
              id: 6,
              email: "test6@test1.com".to_string(),
          },
          User {
              id: 7,
              email: "test677@test1.com".to_string(),
          },
      ]);
    }
    