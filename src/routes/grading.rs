use mongodb::{
    bson::doc,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Database,
};
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

#[get("/<id>")]
pub async fn get_grading(db: &State<Database>, id: String) -> Result<Json<Grading>, Status> {
    let repository = GradingRepository::init(db);

    let result = repository.get_grading(&id).await;

    match result {
        Ok(grading) => Ok(Json(grading)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/<id>", data = "<grading>")]
pub async fn update_grading(
    db: &State<Database>,
    id: String,
    grading: Json<Grading>,
) -> Result<Json<UpdateResult>, Status> {
    let repository = GradingRepository::init(db);

    let result = repository.update_grading(&id, grading.into_inner()).await;

    match result {
        Ok(grading) => Ok(Json(grading)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/<id>")]
pub async fn delete_grading(
    db: &State<Database>,
    id: String,
) -> Result<Json<DeleteResult>, Status> {
    let repository = GradingRepository::init(db);

    let result = repository.delete_grading(&id).await;

    match result {
        Ok(grading) => Ok(Json(grading)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/")]
pub async fn get_all_gradings(db: &State<Database>) -> Result<Json<Vec<Grading>>, Status> {
    let repository = GradingRepository::init(db);

    let result = repository.get_all_gradings().await;

    match result {
        Ok(gradings) => Ok(Json(gradings)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub fn get_all() -> Vec<rocket::Route> {
    routes![
        create_grading,
        get_grading,
        update_grading,
        delete_grading,
        get_all_gradings
    ]
}
