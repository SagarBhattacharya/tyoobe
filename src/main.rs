mod utils;

use std::sync::Arc;
use tokio::net::TcpListener;
use axum::Router;
use mongodb::{Client, Database};
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;

#[derive(Clone)]
pub struct AppState {
  pub db: Database,
}

async fn connect_to_database() -> Database {
  let client = Client::with_uri_str(
    dotenvy::var("DB")
      .unwrap_or("mongodb://localhost:27017".to_string())
  ).await
   .expect("MongoDB connection failed");

  client.database(
    &dotenvy::var("DB_NAME")
      .expect("No Database Name Provided")
  )
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  // load environment variables
  dotenvy::from_filename("dev.env").ok();

  // connect database
  let db = connect_to_database().await;

  // create app
  let app = Router::new()
    // cors layer
    .layer(CorsLayer::new().allow_origin(Any).allow_credentials(true))
    // body limit request layer
    .layer(RequestBodyLimitLayer::new(16 * 1024))
    // state registration
    .with_state(Arc::new(AppState { db }));

  let listener = TcpListener::bind("localhost:3000").await?;

  // listen to requests
  println!("server is listening on: {}", listener.local_addr()?);
  axum::serve(listener, app).await
}
