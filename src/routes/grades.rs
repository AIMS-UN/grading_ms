use mongodb::{bson::doc, Collection, Database};
use rocket::{
    http::Status,
    serde::json::{serde_json::json, Json},
    State,
};

use crate::{
    database::Repository,
    helpers::{response::ApiResponse, serializer::object_id_serializer},
    models::{category::Category, grade::Grade},
};

fn get_grade_repo(db: &State<Database>) -> Repository<Grade> {
    let collection: Collection<Grade> = db.collection("grades");
    Repository::init(collection)
}

fn get_category_repo(db: &State<Database>) -> Repository<Category> {
    let collection: Collection<Category> = db.collection("categories");
    Repository::init(collection)
}

#[post("/", data = "<new_grade>")]
pub async fn create_grade(db: &State<Database>, new_grade: Json<Grade>) -> ApiResponse {
    let result = get_category_repo(db).get(&new_grade.category_id).await;

    match result {
        Ok(result) => match result {
            Some(_) => {
                let result = get_grade_repo(db).create(new_grade.into_inner()).await;

                match result {
                    Ok(grade) => ApiResponse {
                        status: Status::Created,
                        json: Some(json!({ "data": object_id_serializer(&json!(grade)) })),
                    },
                    Err(e) => ApiResponse {
                        status: Status::InternalServerError,
                        json: Some(json!({ "error": e.to_string() })),
                    },
                }
            }
            None => ApiResponse {
                status: Status::NotFound,
                json: Some(json!({ "error": "Category not found" })),
            },
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
                json: Some(json!({ "data": object_id_serializer(&json!(grade)) })),
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

#[get("/?<student_id>&<category_id>")]
pub async fn get_grades(
    db: &State<Database>,
    student_id: Option<String>,
    category_id: Option<String>,
) -> ApiResponse {
    let filter = match (student_id, category_id) {
        (Some(student_id), Some(category_id)) => {
            Some(doc! { "student_id": student_id, "category_id": category_id })
        }
        (Some(student_id), None) => Some(doc! { "student_id": student_id }),
        (None, Some(category_id)) => Some(doc! { "category_id": category_id }),
        (None, None) => None,
    };
    let result = get_grade_repo(db).get_all(filter).await;

    match result {
        Ok(grades) => ApiResponse {
            status: Status::Ok,
            json: Some(json!({ "data": object_id_serializer(&json!(grades)) })),
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
                json: Some(json!({ "data": object_id_serializer(&json!(grade)) })),
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
                status: Status::Gone,
                json: Some(json!({ "data": object_id_serializer(&json!(grade)) })),
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

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Grades", |rocket| async {
        rocket.mount(
            "/api/v1/grades",
            routes![
                create_grade,
                get_grade,
                get_grades,
                update_grade,
                delete_grade
            ],
        )
    })
}
