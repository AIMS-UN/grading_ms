#[macro_use]
extern crate rocket;

mod database;
mod models;
mod routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let db = database::get_db().await;
    let _rocket = rocket::build()
        .manage(db)
        .mount("/", routes::grading::get_all())
        .launch()
        .await?;

    Ok(())
}
