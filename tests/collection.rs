#[cfg(test)]
mod tests {
    use dataverse::{
        client::BaseClient,
        native_api::collection::create::{create_collection, CollectionCreateBody},
        native_api::collection::delete::delete_collection,
        native_api::collection::publish::publish_collection,
        response::Status,
    };
    use lazy_static::lazy_static;

    lazy_static! {
        static ref BASE_URL: String = std::env::var("BASE_URL")
            .ok()
            .expect("BASE_URL must be set for tests");
    }

    lazy_static! {
        static ref API_TOKEN: String = std::env::var("API_TOKEN")
            .ok()
            .expect("API_TOKEN must be set for tests");
    }

    #[test]
    fn test_create_publish_delete_collection() {
        // Part 1: Create a collection
        // Arrange
        let client = BaseClient::new(&BASE_URL.to_string(), Some(&API_TOKEN.to_string())).unwrap();
        let parent = "Root".to_string();
        let body = serde_json::from_str::<CollectionCreateBody>(
            r#"{
            "name": "Test Collection Create",
            "alias": "test_create_collection",
            "description": "Test",
            "dataverseType": "TEACHING_COURSES",
            "affiliation": "University of Dataverse",
            "dataverseContacts": [
              {
                "contactEmail": "john@doe.com"
              }
            ]
            }"#,
        )
        .expect("Could not parse body");

        // Act
        let response =
            create_collection(&client, &parent, &body).expect("Could not create collection");

        // Assert
        assert_eq!(response.status, Status::OK);

        // Part 2: Publish the collection
        let response = publish_collection(&client, &"test_create_collection".to_string())
            .expect("Could not publish collection");

        assert_eq!(response.status, Status::OK, "Publish collection failed");

        // Part 2: Delete the collection
        let response = delete_collection(&client, &"test_create_collection".to_string())
            .expect("Could not delete collection");

        assert_eq!(response.status, Status::OK, "Delete collection failed");
    }
}
