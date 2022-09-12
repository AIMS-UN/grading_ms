use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

#[macro_use]
extern crate rocket;

mod database;
mod models;
mod repositories;
mod routes;

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/api/v1/openapi.json".to_string(),
        ..Default::default()
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let db = database::get_db().await;
    let _rocket = rocket::build()
        .manage(db)
        .mount("/api/v1", routes::grading::get_all())
        .mount("/swagger", make_swagger_ui(&get_docs()))
        .launch()
        .await?;

    Ok(())
}
