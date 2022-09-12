use mongodb::bson::oid::ObjectId;
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyObjectID(ObjectId);

impl JsonSchema for MyObjectID {
    fn schema_name() -> String {
        "MyObjectID".to_string()
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let mut schema = schemars::schema::SchemaObject::default();
        schema.metadata = Some(Box::new(schemars::schema::Metadata {
            description: Some("MyObjectID".to_string()),
            ..Default::default()
        }));
        schema.into()
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Grading {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<MyObjectID>,
    pub concept: String,
    pub grade: f64,
    pub weight: f64,
    pub student_id: String,
    pub group_id: String,
}
