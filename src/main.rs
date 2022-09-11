#[macro_use]
extern crate rocket;

use mongodb::{bson::doc, Client, Database};
use rocket::{http::Status, serde::json::Json};
use std::env;

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json("Hello from rust an rocket".to_string()))
}

#[get("/<name>")]
async fn hello_name(db: &rocket::State<Database>, name: String) -> Result<Json<String>, Status> {
    let collection = db.collection("hello");
    let doc = doc! { "name": name.clone() };
    let result = collection.insert_one(doc, None).await;
    match result {
        Ok(_) => Ok(Json(format!("Hello {}", name))),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let client = Client::with_uri_str(&uri)
        .await
        .expect("Failed to initialize client.");

    let db = client.database("hello_db");

    db.run_command(doc! {"ping": 1}, None)
        .await
        .expect("Failed to ping database.");

    let _rocket = rocket::build()
        .manage(db)
        .mount("/", routes![hello, hello_name])
        .launch()
        .await?;

    Ok(())
}
