#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use dataverse::{
        client::BaseClient,
        native_api::collection::create::{CollectionCreateBody, create_collection},
        native_api::collection::delete::delete_collection,
        native_api::collection::publish::publish_collection,
        response::Status,
    };

    lazy_static! {
        static ref BASE_URL: String = std::env::var("BASE_URL")
            .expect("BASE_URL must be set for tests");
    }

    lazy_static! {
        static ref API_TOKEN: String = std::env::var("API_TOKEN")
            .expect("API_TOKEN must be set for tests");
    }

    #[tokio::test]
    async fn test_create_publish_delete_collection() {
        // Part 1: Create a collection
        // Arrange
        let client = BaseClient::new(&BASE_URL, Some(&API_TOKEN)).unwrap();
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
        let response = create_collection(&client, &parent, body)
            .await
            .expect("Could not create collection");

        // Assert
        assert_eq!(response.status, Status::OK);

        // Part 2: Publish the collection
        let response = publish_collection(&client, "test_create_collection")
            .await
            .expect("Could not publish collection");

        assert_eq!(response.status, Status::OK, "Publish collection failed");

        // Part 2: Delete the collection
        let response = delete_collection(&client, "test_create_collection")
            .await
            .expect("Could not delete collection");

        assert_eq!(response.status, Status::OK, "Delete collection failed");
    }
}
