use mongodb::{Client, Database};

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
async fn main() {
  // load environment variables
  dotenvy::from_filename("dev.env").ok();

  // connect database
  let db = connect_to_database().await;
}
