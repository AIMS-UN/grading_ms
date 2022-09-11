use crate::models::grading::Grading;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection, Database,
};
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

    pub async fn get_grading(&self, id: &str) -> Result<Grading, Error> {
        let oid = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": oid};
        let result = self
            .collection
            .find_one(filter, None)
            .await
            .ok()
            .expect("Failed to get grading");
        Ok(result.unwrap())
    }

    pub async fn update_grading(&self, id: &str, grading: Grading) -> Result<UpdateResult, Error> {
        let oid = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": oid};
        let update = doc! {
            "$set":
            {
                "concept": grading.concept,
                "grade": grading.grade,
                "weight": grading.weight,
                "student_id": grading.student_id,
                "group_id": grading.group_id,
            }
        };
        let result = self
            .collection
            .update_one(filter, update, None)
            .await
            .ok()
            .expect("Failed to update grading");
        Ok(result)
    }

    pub async fn delete_grading(&self, id: &str) -> Result<DeleteResult, Error> {
        let oid = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": oid};
        let result = self
            .collection
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Failed to delete grading");
        Ok(result)
    }
}
