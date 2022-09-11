use crate::models::grading::Grading;
use mongodb::{bson::extjson::de::Error, results::InsertOneResult, Collection, Database};
use rocket::State;

pub struct GradingRepository {
    collection: Collection<Grading>,
}

impl GradingRepository {
    pub fn init(db: &State<Database>) -> Self {
        let collection = db.collection("gradings");
        Self { collection }
    }

    pub async fn create_grading(&self, grading: Grading) -> Result<InsertOneResult, Error> {
        let result = self
            .collection
            .insert_one(grading, None)
            .await
            .ok()
            .expect("Failed to insert grading");
        Ok(result)
    }
}
