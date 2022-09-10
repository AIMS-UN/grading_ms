#[macro_use]
extern crate rocket;

use rocket::{http::Status, serde::json::Json};

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json("Hello from rust an rocket".to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
