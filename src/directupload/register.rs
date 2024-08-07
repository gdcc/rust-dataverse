use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use typify::import_types;

use crate::client::{BaseClient, evaluate_response};
use crate::request::RequestType;
use crate::response::Response;

import_types!(
    schema = "models/directupload/upload.json",
    struct_builder=true,
    derives=[Default, Debug, PartialEq],
);

/// Registers a file for direct upload to a dataset identified by a persistent identifier (PID).
///
/// This asynchronous function sends a POST request to the API endpoint designated for adding files to a dataset.
/// The function constructs the API endpoint URL dynamically, incorporating the dataset's PID. It serializes the
/// provided upload body into JSON format and sets up the request context for a multipart request.
///
/// # Arguments
///
/// * `client` - A reference to the `BaseClient` instance used to send the request.
/// * `pid` - A string slice that holds the persistent identifier of the dataset to which the file will be added.
/// * `body` - The `DirectUploadBody` struct instance containing the file metadata and other relevant information.
///
/// # Returns
///
/// A `Result` wrapping a `Response`, which contains the HTTP response status and the response data if the request is successful,
/// or an `Error` on failure.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use dataverse::prelude::*;
/// use dataverse::directupload::register;
/// use dataverse::client::BaseClient; 
/// use dataverse::directupload::register_file;
///
/// #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let api_token = "api_token".to_string();
/// let base_url = "https://demo.dataverse.com".to_string();
/// let client = BaseClient::new(&base_url, Some(&api_token))
///     .expect("Failed to create client");/// 
///
/// let pid = "doi:10.5072/FK2/QJ8MRH";
///
/// let body = register::DirectUploadBody {
///         categories: vec!["DATA".to_string()],
///         checksum: register::Checksum {
///             type_: "MD5".to_string().into(),
///             value: "d41d8cd98f00b204e9800998ecf8427e".to_string().into(),
///         }.into(),
///         description: "Some description".to_string().into(),
///         directory_label: "some/path".to_string().into(),
///         file_name: "file.txt".to_string().into(),
///         mime_type: "text/plain".to_string().into(),
///         restrict: false.into(),
///         storage_identifier: "s3://bucket/file.txt".to_string().into(),
///     };
///
///     match register_file(&client, pid, body).await {
///         Ok(response) => println!("File registered successfully: {:?}", response),
///         Err(e) => println!("Failed to register file: {}", e),
///     }
///
///     Ok(())
/// }
/// ```
pub async fn register_file(
    client: &BaseClient,
    pid: &str,
    body: DirectUploadBody,
) -> Result<Response<DirectUploadResponse>, String> {
    // Endpoint metadata
    let url = "/api/datasets/:persistentId/add";

    // Set up parameters and the request context
    let parameters = HashMap::from([("persistentId".to_string(), pid.to_string())]);
    let body = HashMap::from([("jsonData".to_string(), serde_json::to_string(&body).unwrap())]);
    let context = RequestType::Multipart {
        bodies: Some(body),
        files: None,
        callbacks: None,
    };

    let response = client.post(url, parameters.into(), &context).await;

    evaluate_response::<DirectUploadResponse>(response).await
}

/// Registers multiple files for direct upload to a dataset identified by a persistent identifier (PID).
///
/// This asynchronous function sends a POST request to the API endpoint designated for adding multiple files to a dataset.
/// The function constructs the API endpoint URL dynamically, incorporating the dataset's PID. It serializes the
/// provided upload bodies into JSON format and sets up the request context for a multipart request.
///
/// # Arguments
///
/// * `client` - A reference to the `BaseClient` instance used to send the request.
/// * `pid` - A string slice that holds the persistent identifier of the dataset to which the files will be added.
/// * `body` - A vector of `DirectUploadBody` struct instances containing the files' metadata and other relevant information.
///
/// # Returns
///
/// A `Result` wrapping a `Response`, which contains the HTTP response status and the response data if the request is successful,
/// or an `Error` on failure.
pub async fn register_multiple_files(
    client: &BaseClient,
    pid: &str,
    body: Vec<DirectUploadBody>,
) -> Result<Response<DirectUploadResponse>, String> {
    // Endpoint metadata
    let url = "/api/datasets/:persistentId/addFiles";

    // Set up parameters and the request context
    let parameters = HashMap::from([("persistentId".to_string(), pid.to_string())]);
    let body = HashMap::from([("jsonData".to_string(), serde_json::to_string(&body).unwrap())]);
    let context = RequestType::Multipart {
        bodies: Some(body),
        files: None,
        callbacks: None,
    };

    let response = client.post(url, parameters.into(), &context).await;

    evaluate_response::<DirectUploadResponse>(response).await
}