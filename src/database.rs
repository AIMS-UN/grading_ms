use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    Client, Collection, Database,
};
use rocket::futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::env;

pub async fn get_db() -> Database {
    let uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let client = Client::with_uri_str(&uri)
        .await
        .expect("Failed to initialize client.");

    let db_name = env::var("MONGO_DB").expect("MONGO_DB must be set");
    let db = client.database(&db_name);

    db.run_command(doc! {"ping": 1}, None)
        .await
        .expect("Failed to ping database.");

    db
}

pub struct Repository<T>
where
    T: Serialize + for<'a> Deserialize<'a> + Unpin + std::marker::Send + Sync,
{
    collection: Collection<T>,
}

impl<T> Repository<T>
where
    T: Serialize + for<'a> Deserialize<'a> + Unpin + std::marker::Send + Sync,
{
    pub fn init(collection: Collection<T>) -> Self {
        Self { collection }
    }

    pub async fn get(&self, id: &str) -> Result<Option<T>, Error> {
        let oid = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": oid};

        let result = self.collection.find_one(filter, None).await;

        match result {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn create(&self, item: T) -> Result<T, Error> {
        let result = self.collection.insert_one(item, None).await;

        match result {
            Ok(result) => {
                let oid = result.inserted_id.as_object_id().unwrap();
                let filter = doc! {"_id": oid};
                let result = self.collection.find_one(filter, None).await;

                match result {
                    Ok(result) => Ok(result.unwrap()),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<T>, Error> {
        let cursor = self.collection.find(None, None).await;

        match cursor {
            Ok(cursor) => {
                let items = cursor.map(|item| item.unwrap()).collect::<Vec<T>>().await;

                Ok(items)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn update(&self, id: &str, item: T) -> Result<Option<T>, Error> {
        let oid = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": oid};

        let result = self.collection.replace_one(filter, item, None).await;

        match result {
            Ok(result) => {
                if result.modified_count == 0 {
                    return Ok(None);
                }

                let filter = doc! {"_id": oid};
                let result = self.collection.find_one(filter, None).await;

                match result {
                    Ok(result) => Ok(result),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub async fn delete(&self, id: &str) -> Result<Option<T>, Error> {
        let oid = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": oid};

        let result = self.collection.find_one_and_delete(filter, None).await;

        match result {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }
}
