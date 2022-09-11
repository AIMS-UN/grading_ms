#[macro_use]
extern crate rocket;

mod database;
mod models;
mod repositories;
mod routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let db = database::get_db().await;
    let _rocket = rocket::build()
        .manage(db)
        .mount("/api/v1/grading", routes::grading::get_all())
        .launch()
        .await?;

    Ok(())
}
