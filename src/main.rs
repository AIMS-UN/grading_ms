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
        .attach(routes::categories::stage())
        .attach(routes::grades::stage())
        .launch()
        .await?;

    Ok(())
}
