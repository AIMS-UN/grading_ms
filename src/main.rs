#[macro_use]
extern crate rocket;

mod database;
mod helpers;
mod models;
mod routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let db = database::get_db().await;
    let _rocket = rocket::build()
        .manage(db)
        .mount(
            "/api/v1/categories",
            routes![
                routes::categories::create_category,
                routes::categories::get_category,
                routes::categories::get_categories,
                routes::categories::update_category,
                routes::categories::delete_category
            ],
        )
        .mount(
            "/api/v1/grades",
            routes![
                routes::grades::create_grade,
                routes::grades::get_grade,
                routes::grades::get_grades,
                routes::grades::update_grade,
                routes::grades::delete_grade
            ],
        )
        .launch()
        .await?;

    Ok(())
}
