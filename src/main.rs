#[macro_use]
extern crate rocket;

use mongodb::{bson::doc, sync::Client};
use rocket::{http::Status, serde::json::Json};
use std::env;

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json("Hello from rust an rocket".to_string()))
}

#[launch]
fn rocket() -> _ {
    let uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let client = Client::with_uri_str(&uri).expect("Invalid URI");
    client
        .database("testing_db")
        .collection("testing_collection")
        .insert_one(doc! {"name": "test"}, None)
        .expect("Failed to insert document.");
    rocket::build().mount("/", routes![hello])
}
