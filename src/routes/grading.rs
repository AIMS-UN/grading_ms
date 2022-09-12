use mongodb::{bson::doc, Database};
use rocket::{http::Status, serde::json::Json, State};
use rocket_okapi::{openapi, openapi_get_routes};

use crate::{models::grading::Grading, repositories::grading::GradingRepository};

#[openapi]
#[post("/grading", data = "<new_grading>")]
pub async fn create_grading(
    db: &State<Database>,
    new_grading: Json<Grading>,
) -> Result<Status, Status> {
    let repository = GradingRepository::init(db);

    let result = repository.create_grading(new_grading.into_inner()).await;

    match result {
        Ok(_) => Ok(Status::Created),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[openapi]
#[get("/grading/<id>")]
pub async fn get_grading(db: &State<Database>, id: String) -> Result<Json<Grading>, Status> {
    let repository = GradingRepository::init(db);

    let result = repository.get_grading(&id).await;

    match result {
        Ok(grading) => Ok(Json(grading)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[openapi]
#[put("/grading/<id>", data = "<grading>")]
pub async fn update_grading(
    db: &State<Database>,
    id: String,
    grading: Json<Grading>,
) -> Result<Status, Status> {
    let repository = GradingRepository::init(db);

    let result = repository.update_grading(&id, grading.into_inner()).await;

    match result {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[openapi]
#[delete("/grading/<id>")]
pub async fn delete_grading(db: &State<Database>, id: String) -> Result<Status, Status> {
    let repository = GradingRepository::init(db);

    let result = repository.delete_grading(&id).await;

    match result {
        Ok(_) => Ok(Status::Ok),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[openapi]
#[get("/grading")]
pub async fn get_all_gradings(db: &State<Database>) -> Result<Json<Vec<Grading>>, Status> {
    let repository = GradingRepository::init(db);

    let result = repository.get_all_gradings().await;

    match result {
        Ok(gradings) => Ok(Json(gradings)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub fn get_all() -> Vec<rocket::Route> {
    openapi_get_routes![
        create_grading,
        get_grading,
        update_grading,
        delete_grading,
        get_all_gradings
    ]
}
