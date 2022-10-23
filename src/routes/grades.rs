use mongodb::{bson::doc, Collection, Database};
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

fn get_grade_repo(db: &State<Database>) -> Repository<Grade> {
    let collection: Collection<Grade> = db.collection("grades");
    Repository::init(collection)
}

fn get_category_repo(db: &State<Database>) -> Repository<Category> {
    let collection: Collection<Category> = db.collection("categories");
    Repository::init(collection)
}

#[post("/", data = "<new_grade>")]
pub async fn create_grade(
    db: &State<Database>,
    new_grade: Json<Grade>,
) -> status::Custom<serde_json::Value> {
    let result = get_category_repo(db).get(&new_grade.category_id).await;

    match result {
        Ok(Some(_)) => {
            let result = get_grade_repo(db).create(new_grade.into_inner()).await;

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
pub async fn get_grade(db: &State<Database>, id: String) -> status::Custom<serde_json::Value> {
    let result = get_grade_repo(db).get(&id).await;

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
    db: &State<Database>,
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

    let result = get_grade_repo(db).get_all(Some(filter)).await;

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
    db: &State<Database>,
    id: String,
    updated_grade: Json<Grade>,
) -> status::Custom<serde_json::Value> {
    let result = get_grade_repo(db)
        .update(&id, updated_grade.into_inner())
        .await;

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
pub async fn delete_grade(db: &State<Database>, id: String) -> status::Custom<serde_json::Value> {
    let result = get_grade_repo(db).delete(&id).await;

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
