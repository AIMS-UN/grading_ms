#[macro_use]
extern crate rocket;

use mongodb::{bson::doc, Client};
use rocket::{http::Status, serde::json::Json};
use std::env;

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json("Hello from rust an rocket".to_string()))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let client = Client::with_uri_str(&uri)
        .await
        .expect("Failed to initialize client.");

    client
        .database("testing_db")
        .collection("testing_collection")
        .insert_one(doc! {"name": "test"}, None)
        .await
        .expect("Failed to insert document.");

    let _rocket = rocket::build().mount("/", routes![hello]).launch().await?;

    Ok(())
}
