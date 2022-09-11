use crate::models::grading::Grading;
use mongodb::{Collection, Database};
use rocket::State;

pub struct GradingRepository {
    collection: Collection<Grading>,
}

impl GradingRepository {
    pub fn init(db: &State<Database>) -> Self {
        let collection = db.collection("gradings");
        Self { collection }
    }
}
