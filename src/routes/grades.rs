use crate::database::Repository;
use crate::helpers::response::ApiResponse;
use crate::models::grade::Grade;

use mongodb::{bson::doc, Collection, Database};
use rocket::{
    http::Status,
    serde::json::{serde_json::json, Json},
    State,
};

fn get_grade_repo(db: &State<Database>) -> Repository<Grade> {
    let collection: Collection<Grade> = db.collection("grades");
    Repository::init(collection)
}

#[post("/", data = "<new_grade>")]
pub async fn create_grade(db: &State<Database>, new_grade: Json<Grade>) -> ApiResponse {
    let result = get_grade_repo(db).create(new_grade.into_inner()).await;

    match result {
        Ok(grade) => ApiResponse {
            status: Status::Created,
            json: Some(json!({ "data": grade })),
        },
        Err(e) => ApiResponse {
            status: Status::InternalServerError,
            json: Some(json!({ "error": e.to_string() })),
        },
    }
}

#[get("/<id>")]
pub async fn get_grade(db: &State<Database>, id: String) -> ApiResponse {
    let result = get_grade_repo(db).get(&id).await;

    match result {
        Ok(result) => match result {
            Some(grade) => ApiResponse {
                status: Status::Ok,
                json: Some(json!({ "data": grade })),
            },
            None => ApiResponse {
                status: Status::NotFound,
                json: Some(json!({ "error": "Grade not found" })),
            },
        },
        Err(e) => ApiResponse {
            status: Status::InternalServerError,
            json: Some(json!({ "error": e.to_string() })),
        },
    }
}

#[get("/")]
pub async fn get_grades(db: &State<Database>) -> ApiResponse {
    let result = get_grade_repo(db).get_all().await;

    match result {
        Ok(grades) => ApiResponse {
            status: Status::Ok,
            json: Some(json!({ "data": grades })),
        },
        Err(e) => ApiResponse {
            status: Status::InternalServerError,
            json: Some(json!({ "error": e.to_string() })),
        },
    }
}

#[put("/<id>", data = "<updated_grade>")]
pub async fn update_grade(
    db: &State<Database>,
    id: String,
    updated_grade: Json<Grade>,
) -> ApiResponse {
    let result = get_grade_repo(db)
        .update(&id, updated_grade.into_inner())
        .await;

    match result {
        Ok(result) => match result {
            Some(grade) => ApiResponse {
                status: Status::Ok,
                json: Some(json!({ "data": grade })),
            },
            None => ApiResponse {
                status: Status::NotFound,
                json: Some(json!({ "error": "Grade not found" })),
            },
        },
        Err(e) => ApiResponse {
            status: Status::InternalServerError,
            json: Some(json!({ "error": e.to_string() })),
        },
    }
}

#[delete("/<id>")]
pub async fn delete_grade(db: &State<Database>, id: String) -> ApiResponse {
    let result = get_grade_repo(db).delete(&id).await;

    match result {
        Ok(result) => match result {
            Some(grade) => ApiResponse {
                status: Status::Ok,
                json: Some(json!({ "data": grade })),
            },
            None => ApiResponse {
                status: Status::NotFound,
                json: Some(json!({ "error": "Grade not found" })),
            },
        },
        Err(e) => ApiResponse {
            status: Status::InternalServerError,
            json: Some(json!({ "error": e.to_string() })),
        },
    }
}
