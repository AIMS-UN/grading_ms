use mongodb::{bson::doc, Client, Database};
use std::env;

pub async fn get_db() -> Database {
    let uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let client = Client::with_uri_str(&uri)
        .await
        .expect("Failed to initialize client.");

    let db = client.database("gradings_db");

    db.run_command(doc! {"ping": 1}, None)
        .await
        .expect("Failed to ping database.");

    db
}
