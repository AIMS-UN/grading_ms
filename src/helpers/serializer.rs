use rocket::serde::json::serde_json;

pub fn object_id_serializer(json: &serde_json::Value) -> serde_json::Value {
    match json {
        serde_json::Value::Object(obj) => {
            if !obj.contains_key("_id") {
                return json.clone();
            }

            let id = obj
                .get("_id")
                .unwrap()
                .get("$oid")
                .unwrap()
                .as_str()
                .unwrap();

            let mut new_obj = obj.clone();
            new_obj.insert("id".to_string(), serde_json::Value::String(id.to_string()));
            new_obj.remove("_id");

            serde_json::Value::Object(new_obj)
        }
        serde_json::Value::Array(arr) => {
            let mut new_arr = Vec::new();

            for item in arr {
                new_arr.push(object_id_serializer(item));
            }

            serde_json::Value::Array(new_arr)
        }
        _ => json.clone(),
    }
}

#[cfg(test)]
mod tests {
    use mongodb::bson::oid::ObjectId;
    use rocket::serde::json::serde_json::json;

    use crate::models::category::Category;

    use super::*;

    #[test]
    fn test_object_id_serializer() {
        let json = json!({
            "_id": { "$oid": "5f9f1b0b0b1b4b0b1b0b0b0b" },
            "name": "test",
        });

        let expected = json!({
            "id": "5f9f1b0b0b1b4b0b1b0b0b0b",
            "name": "test",
        });

        let result = object_id_serializer(&json);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_object_id_serializer_no_id() {
        let json = json!({
            "name": "test",
        });

        let expected = json!({
            "name": "test",
        });

        let result = object_id_serializer(&json);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_object_id_serializer_category() {
        let json = json!(Category {
            id: Some(ObjectId::parse_str("5f9f1b0b0b9b9b0b0b0b0b0b").unwrap()),
            name: "test".to_string(),
            weight: 4.0,
            subject_code: "1000003-B".to_string(),
            group_id: "ef9f1b0b0b9b9b0b0b0b0b0b".to_string(),
        });

        let expected = json!({
            "id": "5f9f1b0b0b9b9b0b0b0b0b0b",
            "name": "test",
            "weight": 4.0,
            "subject_code": "1000003-B",
            "group_id": "ef9f1b0b0b9b9b0b0b0b0b0b",
        });

        let result = object_id_serializer(&json);

        assert_eq!(result, expected);
    }

    // test array
    #[test]
    fn test_object_id_serializer_array() {
        let json = json!([
            {
                "_id": { "$oid": "5f9f1b0b0b1b4b0b1b0b0b0b" },
                "name": "test",
            },
            {
                "_id": { "$oid": "5f9f1b0b0b1b4b0b1b0b0b0c" },
                "name": "test",
            },
        ]);

        let expected = json!([
            {
                "id": "5f9f1b0b0b1b4b0b1b0b0b0b",
                "name": "test",
            },
            {
                "id": "5f9f1b0b0b1b4b0b1b0b0b0c",
                "name": "test",
            },
        ]);

        let result = object_id_serializer(&json);

        assert_eq!(result, expected);
    }
}
