use mongodb::bson::doc;
use rocket::{
    http::Status,
    response::status,
    serde::json::{
        serde_json::{self, json},
        Json,
    },
    State,
};

use crate::{
    database::Repository,
    helpers::serializer::object_id_serializer,
    models::{category::Category, grade::Grade},
};

#[post("/", data = "<new_grade>")]
pub async fn create_grade(
    grade_repo: &State<Repository<Grade>>,
    category_repo: &State<Repository<Category>>,
    new_grade: Json<Grade>,
) -> status::Custom<serde_json::Value> {
    let result = category_repo.get(&new_grade.category_id).await;

    match result {
        Ok(Some(_)) => {
            let result = grade_repo.create(new_grade.into_inner()).await;

            match result {
                Ok(grade) => status::Custom(
                    Status::Created,
                    json!({ "data": object_id_serializer(&json!(grade)) }),
                ),
                Err(e) => status::Custom(
                    Status::InternalServerError,
                    json!({ "error": e.to_string() }),
                ),
            }
        }
        Ok(None) => status::Custom(Status::NotFound, json!({ "error": "Category not found" })),
        Err(e) => status::Custom(
            Status::InternalServerError,
            json!({ "error": e.to_string() }),
        ),
    }
}

#[get("/<id>")]
pub async fn get_grade(
    grade_repo: &State<Repository<Grade>>,
    id: String,
) -> status::Custom<serde_json::Value> {
    let result = grade_repo.get(&id).await;

    match result {
        Ok(Some(grade)) => status::Custom(
            Status::Ok,
            json!({ "data": object_id_serializer(&json!(grade)) }),
        ),
        Ok(None) => status::Custom(Status::NotFound, json!({ "error": "Grade not found" })),
        Err(e) => status::Custom(
            Status::InternalServerError,
            json!({ "error": e.to_string() }),
        ),
    }
}

#[get("/?<student_id>&<category_id>")]
pub async fn get_grades(
    grade_repo: &State<Repository<Grade>>,
    student_id: Option<String>,
    category_id: Option<String>,
) -> status::Custom<serde_json::Value> {
    let mut filter = doc! {};
    if let Some(student_id) = student_id {
        filter.insert("student_id", student_id);
    }
    if let Some(category_id) = category_id {
        filter.insert("category_id", category_id);
    }

    let result = grade_repo.get_all(Some(filter)).await;

    match result {
        Ok(grades) => status::Custom(
            Status::Ok,
            json!({ "data": object_id_serializer(&json!(grades)) }),
        ),
        Err(e) => status::Custom(
            Status::InternalServerError,
            json!({ "error": e.to_string() }),
        ),
    }
}

#[put("/<id>", data = "<updated_grade>")]
pub async fn update_grade(
    grade_repo: &State<Repository<Grade>>,
    id: String,
    updated_grade: Json<Grade>,
) -> status::Custom<serde_json::Value> {
    let result = grade_repo.update(&id, updated_grade.into_inner()).await;

    match result {
        Ok(Some(grade)) => status::Custom(
            Status::Ok,
            json!({ "data": object_id_serializer(&json!(grade)) }),
        ),
        Ok(None) => status::Custom(Status::NotFound, json!({ "error": "Grade not found" })),
        Err(e) => status::Custom(
            Status::InternalServerError,
            json!({ "error": e.to_string() }),
        ),
    }
}

#[delete("/<id>")]
pub async fn delete_grade(
    grade_repo: &State<Repository<Grade>>,
    id: String,
) -> status::Custom<serde_json::Value> {
    let result = grade_repo.delete(&id).await;

    match result {
        Ok(Some(grade)) => status::Custom(
            Status::Gone,
            json!({ "data": object_id_serializer(&json!(grade)) }),
        ),
        Ok(None) => status::Custom(Status::NotFound, json!({ "error": "Grade not found" })),
        Err(e) => status::Custom(
            Status::InternalServerError,
            json!({ "error": e.to_string() }),
        ),
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
