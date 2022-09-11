use mongodb::{bson::doc, results::InsertOneResult, Database};
use rocket::{http::Status, serde::json::Json, State};

use crate::{models::grading::Grading, repositories::grading::GradingRepository};

#[post("/", data = "<new_grading>")]
pub async fn create_grading(
    db: &State<Database>,
    new_grading: Json<Grading>,
) -> Result<Json<InsertOneResult>, Status> {
    let repository = GradingRepository::init(db);

    let result = repository.create_grading(new_grading.into_inner()).await;

    match result {
        Ok(grading) => Ok(Json(grading)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub fn get_all() -> Vec<rocket::Route> {
    routes![create_grading]
}
