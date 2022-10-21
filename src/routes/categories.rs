use mongodb::{bson::doc, Collection, Database};
use rocket::{
    http::Status,
    serde::json::{serde_json::json, Json},
    State,
};

use crate::{
    database::Repository,
    helpers::{response::ApiResponse, serializer::object_id_serializer},
    models::category::Category,
};

fn get_category_repo(db: &State<Database>) -> Repository<Category> {
    let collection: Collection<Category> = db.collection("categories");
    Repository::init(collection)
}

#[post("/", data = "<new_category>")]
pub async fn create_category(db: &State<Database>, new_category: Json<Category>) -> ApiResponse {
    let result = get_category_repo(db)
        .create(new_category.into_inner())
        .await;

    match result {
        Ok(category) => ApiResponse {
            status: Status::Created,
            json: Some(json!({ "data": object_id_serializer(&json!(category)) })),
        },
        Err(e) => ApiResponse {
            status: Status::InternalServerError,
            json: Some(json!({ "error": e.to_string() })),
        },
    }
}

#[get("/<id>")]
pub async fn get_category(db: &State<Database>, id: String) -> ApiResponse {
    let result = get_category_repo(db).get(&id).await;

    match result {
        Ok(result) => match result {
            Some(category) => ApiResponse {
                status: Status::Ok,
                json: Some(json!({ "data": object_id_serializer(&json!(category)) })),
            },
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

#[get("/?<subject_code>&<group_id>")]
pub async fn get_categories(
    db: &State<Database>,
    subject_code: Option<String>,
    group_id: Option<String>,
) -> ApiResponse {
    let mut filter = doc! {};
    if let Some(subject_code) = subject_code {
        filter.insert("subject_code", subject_code);
    }
    if let Some(group_id) = group_id {
        filter.insert("group_id", group_id);
    }

    let result = get_category_repo(db).get_all(Some(filter)).await;

    match result {
        Ok(categories) => ApiResponse {
            status: Status::Ok,
            json: Some(json!({ "data": object_id_serializer(&json!(categories)) })),
        },
        Err(e) => ApiResponse {
            status: Status::InternalServerError,
            json: Some(json!({ "error": e.to_string() })),
        },
    }
}

#[put("/<id>", data = "<updated_category>")]
pub async fn update_category(
    db: &State<Database>,
    id: String,
    updated_category: Json<Category>,
) -> ApiResponse {
    let result = get_category_repo(db)
        .update(&id, updated_category.into_inner())
        .await;

    match result {
        Ok(result) => match result {
            Some(category) => ApiResponse {
                status: Status::Ok,
                json: Some(json!({ "data": object_id_serializer(&json!(category)) })),
            },
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

#[delete("/<id>")]
pub async fn delete_category(db: &State<Database>, id: String) -> ApiResponse {
    let result = get_category_repo(db).delete(&id).await;

    match result {
        Ok(result) => match result {
            Some(category) => ApiResponse {
                status: Status::Gone,
                json: Some(json!({ "data": object_id_serializer(&json!(category)) })),
            },
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

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Categories", |rocket| async {
        rocket.mount(
            "/api/v1/categories",
            routes![
                create_category,
                get_category,
                get_categories,
                update_category,
                delete_category
            ],
        )
    })
}
