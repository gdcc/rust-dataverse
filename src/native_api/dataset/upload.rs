use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json;
use typify::import_types;

use crate::{
    callback::CallbackFun,
    client::{BaseClient, evaluate_response},
    identifier::Identifier,
    request::RequestType,
    response::Response,
};

import_types!(
    schema = "models/file/filemeta.json",
    struct_builder = true,
);

/// Uploads a file to a dataset identified by either a persistent identifier (PID) or a numeric ID.
///
/// This asynchronous function sends a POST request to the API endpoint designated for adding files to a dataset.
/// The function constructs the API endpoint URL dynamically, incorporating the dataset's identifier. It sets up
/// the request context for a multipart request, including the file path, optional body metadata, and optional callback.
///
/// # Arguments
///
/// * `client` - A reference to the `BaseClient` instance used to send the request.
/// * `id` - An `Identifier` enum instance, which can be either a `PersistentId(String)` or an `Id(i64)`,
///          representing the unique identifier of the dataset to which the file will be uploaded.
/// * `fpath` - A `PathBuf` instance representing the file path of the file to be uploaded.
/// * `body` - An optional `UploadBody` struct instance containing additional metadata for the upload.
/// * `callback` - An optional `CallbackFun` instance for handling callbacks during the upload process.
///
/// # Returns
///
/// A `Result` wrapping a `Response<UploadResponse>`, which contains the HTTP response status and the deserialized
/// response data indicating the outcome of the upload operation, if the request is successful, or a `String` error message on failure.
pub async fn upload_file_to_dataset(
    client: &BaseClient,
    id: Identifier,
    fpath: PathBuf,
    body: Option<UploadBody>,
    callback: Option<CallbackFun>,
) -> Result<Response<UploadResponse>, String> {
    // Endpoint metadata
    let path = match id {
        Identifier::PersistentId(_) => "api/datasets/:persistentId/add".to_string(),
        Identifier::Id(id) => format!("api/datasets/{}/add", id),
    };

    // Build hash maps for the request
    let file = HashMap::from([("file".to_string(), fpath)]);
    let callbacks = callback.map(|c| HashMap::from([("file".to_string(), c)]));
    let body = body.as_ref().map(|b| {
        HashMap::from([("jsonData".to_string(), serde_json::to_string(&b).unwrap())])
    });

    // Build the request context
    let context = RequestType::Multipart {
        bodies: body,
        files: Some(file),
        callbacks,
    };

    let response = match id {
        Identifier::PersistentId(id) => client.post(
            path.as_str(),
            Some(HashMap::from([("persistentId".to_string(), id.clone())])),
            &context,
        ),
        Identifier::Id(_) => client.post(path.as_str(), None, &context),
    }.await;

    evaluate_response::<UploadResponse>(response).await
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::identifier::Identifier;
    use crate::prelude::BaseClient;
    use crate::prelude::dataset::upload::upload_file_to_dataset;
    use crate::test_utils::{create_test_dataset, extract_test_env, prepare_upload_body};

    /// Tests the file upload functionality to a dataset using a persistent identifier (PID).
    ///
    /// This test case demonstrates the process of uploading a file to a dataset identified by its PID.
    /// It involves setting up a client with API token and base URL obtained from environment variables,
    /// creating a test dataset within a specified parent dataverse ("Root"), and uploading a file to this dataset.
    /// The test asserts that the file upload operation was successful.
    ///
    /// # Environment Variables
    /// - `API_TOKEN`: The API token used for authentication with the API.
    /// - `BASE_URL`: The base URL of the instance.
    ///
    /// # Panics
    /// This test will panic if the file upload operation fails, indicating an issue with the file upload process.
    #[tokio::test]
    async fn test_upload_file_to_dataset_with_pid() {
        // Set up the client
        let (api_token, base_url, _) = extract_test_env();
        let client = BaseClient::new(&base_url, Some(&api_token)).unwrap();

        // Create a test dataset
        let (_, pid) = create_test_dataset(&client, "Root").await;

        // Create a test file
        let fpath = PathBuf::from("tests/fixtures/file.txt");

        // Upload the file to the dataset
        let response = upload_file_to_dataset(
            &client,
            Identifier::PersistentId(pid),
            fpath,
            None,
            None,
        )
            .await
            .expect("Failed to upload file to dataset");

        // Assert that the upload was successful
        assert!(response.status.is_ok());
    }

    /// Tests the file upload functionality to a dataset using a dataset ID.
    ///
    /// This test case demonstrates the process of uploading a file to a dataset identified by its dataset ID.
    /// It involves setting up a client with API token and base URL obtained from environment variables,
    /// creating a test dataset within a specified parent dataverse ("Root"), and uploading a file to this dataset.
    /// The test asserts that the file upload operation was successful.
    ///
    /// # Environment Variables
    /// - `API_TOKEN`: The API token used for authentication with the API.
    /// - `BASE_URL`: The base URL of the instance.
    ///
    /// # Panics
    /// This test will panic if the file upload operation fails, indicating an issue with the file upload process.
    #[tokio::test]
    async fn test_upload_file_to_dataset_with_id() {
        // Set up the client
        let (api_token, base_url, _) = extract_test_env();
        let client = BaseClient::new(&base_url, Some(&api_token)).unwrap();

        // Create a test dataset
        let (id, _) = create_test_dataset(&client, "Root").await;

        // Create a test file
        let fpath = PathBuf::from("tests/fixtures/file.txt");

        // Upload the file to the dataset
        let response = upload_file_to_dataset(
            &client,
            Identifier::Id(id),
            fpath,
            None,
            None,
        )
            .await
            .expect("Failed to upload file to dataset");

        // Assert that the upload was successful
        assert!(response.status.is_ok());
    }

    /// Tests the file upload functionality to a dataset with additional metadata.
    ///
    /// This test demonstrates uploading a file to a dataset identified by a persistent identifier (PID),
    /// along with additional metadata specified in the body of the request. It sets up a client using API token
    /// and base URL obtained from environment variables, creates a test dataset within a specified parent dataverse
    /// ("Root"), and uploads a file to this dataset with additional metadata. The test asserts that the file upload
    /// operation, including the metadata submission, was successful.
    ///
    /// # Environment Variables
    /// - `API_TOKEN`: The API token used for authentication with the API.
    /// - `BASE_URL`: The base URL of the instance.
    ///
    /// # Panics
    /// This test will panic if the file upload operation fails, indicating an issue with either the file upload
    /// process or the metadata submission.
    #[tokio::test]
    async fn test_upload_with_body() {
        // Set up the client
        let (api_token, base_url, _) = extract_test_env();
        let client = BaseClient::new(&base_url, Some(&api_token)).unwrap();

        // Create a test dataset
        let (_, pid) = create_test_dataset(&client, "Root").await;

        // Create a test file
        let fpath = PathBuf::from("tests/fixtures/file.txt");

        // Create a test body
        let body = prepare_upload_body();

        // Upload the file to the dataset
        let response = upload_file_to_dataset(
            &client,
            Identifier::PersistentId(pid),
            fpath,
            Some(body),
            None,
        )
            .await
            .expect("Failed to upload file to dataset");

        // Assert that the upload was successful
        assert!(response.status.is_ok());
    }

    /// Tests the behavior of the upload functionality with a non-existent file.
    ///
    /// This test aims to verify the system's behavior when attempting to upload a file that does not exist.
    /// It sets up a client using API token and base URL obtained from environment variables, creates a test dataset
    /// within a specified parent dataverse ("Root"), and then attempts to upload a non-existent file to this dataset.
    /// The test is expected to panic, indicating that the system correctly identifies the file as non-existent
    /// and fails the operation.
    ///
    /// # Environment Variables
    /// - `API_TOKEN`: The API token used for authentication with the API.
    /// - `BASE_URL`: The base URL of the instance.
    ///
    /// # Panics
    /// This test will panic, as it is expected to do so when the file upload operation fails due to the file not existing.
    #[tokio::test]
    #[should_panic]
    async fn test_upload_non_existent_file() {
        // Set up the client
        let (api_token, base_url, _) = extract_test_env();
        let client = BaseClient::new(&base_url, Some(&api_token)).unwrap();

        // Create a test dataset
        let (_, pid) = create_test_dataset(&client, "Root").await;

        // Create a test file
        let fpath = PathBuf::from("tests/fixtures/non_existent_file.txt");

        // Upload the file to the dataset
        upload_file_to_dataset(
            &client,
            Identifier::PersistentId(pid),
            fpath,
            None,
            None,
        )
            .await
            .expect("Failed to upload file to dataset");
    }
}
