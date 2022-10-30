#[macro_use]
extern crate rocket;

mod database;
mod helpers;
mod models;
mod routes;

use database::Repository;
use models::{category::Category, grade::Grade};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let db = database::get_db().await.unwrap();

    let category_repo: Repository<Category> = Repository::init(db.collection("categories"));
    let grade_repo: Repository<Grade> = Repository::init(db.collection("grades"));

    let _rocket = rocket::build()
        .manage(category_repo)
        .manage(grade_repo)
        .attach(routes::categories::stage())
        .attach(routes::grades::stage())
        .launch()
        .await?;

    Ok(())
}
