use mongodb::{bson::doc, Database};
use rocket::{http::Status, serde::json::Json};

#[get("/")]
pub fn hello() -> Result<Json<String>, Status> {
    Ok(Json("Hello from rust an rocket".to_string()))
}

#[get("/<name>")]
pub async fn hello_name(
    db: &rocket::State<Database>,
    name: String,
) -> Result<Json<String>, Status> {
    let collection = db.collection("hello");
    let doc = doc! { "name": name.clone() };
    let result = collection.insert_one(doc, None).await;
    match result {
        Ok(_) => Ok(Json(format!("Hello {}", name))),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub fn get_all() -> Vec<rocket::Route> {
    routes![hello, hello_name]
}
