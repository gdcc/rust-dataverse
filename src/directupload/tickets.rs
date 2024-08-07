use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use typify::import_types;

use crate::{
    client::{BaseClient, evaluate_response},
    request::RequestType,
    response::Response,
};
use crate::prelude::CallbackFun;

import_types!(
    schema = "models/directupload/tickets.json",
    struct_builder=true,
);

/// Retrieves an upload ticket for a file.
///
/// This asynchronous function sends a request to the server to obtain an upload ticket
/// for a file, which includes metadata such as the file size and project ID.
///
/// # Arguments
///
/// * `client` - A reference to the `BaseClient` used for making API requests.
/// * `pid` - A string slice representing the project ID.
/// * `size` - The size of the file in bytes.
///
/// # Returns
///
/// A `Result` wrapping a `Response<TicketResponse>` if the request is successful, or a `String` error message if it fails.
pub async fn get_ticket(
    client: &BaseClient,
    pid: &str,
    size: usize,
) -> Result<Response<TicketResponse>, String> {
    // Endpoint metadata
    let url = "/api/datasets/:persistentId/uploadurls";

    // Build Parameters
    let parameters = Some(HashMap::from([
        ("persistentId".to_string(), pid.to_owned()),
        ("size".to_string(), size.to_string()),
    ]));

    let context = RequestType::Plain;
    let response = client.get(url, parameters, &context).await;

    evaluate_response::<TicketResponse>(response).await
}

/// Processes an upload ticket and uploads the file.
///
/// This asynchronous function performs the following steps:
/// 1. Unwraps the ticket data to retrieve the storage identifier.
/// 2. Checks if the ticket is for a multipart upload.
/// 3. If not multipart, performs a single-part upload of the file.
/// 4. Returns the storage identifier if the upload is successful.
///
/// # Arguments
///
/// * `ticket` - A `Response<TicketResponse>` containing the upload ticket data.
/// * `filepath` - A reference to a `Path` representing the path to the file to be uploaded.
/// * `callback` - An optional `CallbackFun` instance for handling callbacks during the upload process.
///
/// # Returns
///
/// A `Result` wrapping an `Option<String>` containing the storage identifier if the upload is successful, or an error if it fails.
pub async fn process_ticket(
    ticket: Response<TicketResponse>,
    filepath: &Path,
    callback: Option<CallbackFun>,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let ticket = ticket.data.unwrap();
    let storage_identifier = ticket
        .storage_identifier
        .clone();

    if !is_multipart(&ticket) {
        single_part_upload(ticket, filepath, callback)
            .await?;
        Ok(storage_identifier)
    } else {
        Err("Multipart upload not supported yet".into())
    }
}

fn is_multipart(ticket: &TicketResponse) -> bool {
    ticket.url.is_none()
}

/// Uploads a file to S3 in a single part.
///
/// This asynchronous function performs the following steps:
/// 1. Creates a new `BaseClient` using the URL from the ticket.
/// 2. Adds necessary headers to the client, including the file size and tagging information.
/// 3. Constructs a `RequestType::File` context with the file path and optional callback function.
/// 4. Sends a PUT request to upload the file to S3.
///
/// # Arguments
///
/// * `ticket` - A `TicketResponse` containing the upload ticket data.
/// * `filepath` - A reference to a `Path` representing the path to the file to be uploaded.
/// * `callback_fun` - An optional `CallbackFun` instance for handling callbacks during the upload process.
///
/// # Returns
///
/// A `Result` indicating success or failure. On success, returns `Ok(())`. On failure, returns an error wrapped in a `Box<dyn std::error::Error>`.
async fn single_part_upload(
    ticket: TicketResponse,
    filepath: &Path,
    callback_fun: Option<CallbackFun>,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = parse_s3_url(ticket.url);
    let mut client = BaseClient::new(
        &url,
        None,
    )?;

    // Add the headers to the client
    let file_size = std::fs::metadata(filepath)?.len();
    client.add_header("x-amz-tagging", "dv-state=temp");
    client.add_header("Content-Length", file_size.to_string().as_str());

    let context = RequestType::File {
        file: filepath.to_path_buf(),
        callback: callback_fun,
    };

    match client.put("", None, &context).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}


fn parse_s3_url(url: Option<String>) -> String {
    let mut url = url.expect("S3 URL should not be None");

    // While testing, the URL might start with "http://localstack"
    // due to DV using the Docker-URL for localstack which is not
    // accessible from the host machine. Turning it into "localhost"
    // makes it accessible.
    if url.starts_with("http://localstack") {
        url = url.replace("http://localstack", "http://localhost");
    }

    url
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::prelude::*;
    use crate::test_utils::{create_test_dataset, extract_test_env};

    #[tokio::test]
    async fn test_get_tickets_small_file() {
        // Run test only if the environment variables are set
        let alias = env::var("DIRECT_UPLOAD_COLLECTION");
        if alias.is_err() {
            println!("Skipping direct upload test");
            return;
        }

        let (api_token, base_url, _) = extract_test_env();
        let client = BaseClient::new(&base_url, Some(&api_token))
            .expect("Failed to create client");

        // Create a test dataset
        let (_, pid) = create_test_dataset(&client, &alias.unwrap()).await;

        let response = directupload::get_ticket(
            &client,
            pid.as_str(),
            10 * 1024, // 10KB
        ).await;

        assert!(response.is_ok(), "Failed to get tickets for small file");

        let data = response.unwrap().data.unwrap();
        assert!(data.urls.is_empty(), "URLs should be empty for small files");
        assert!(data.url.is_some(), "URL should not be None for small files");
    }

    #[tokio::test]
    async fn test_tickets_big_file() {
        // Run test only if the environment variables are set
        let alias = env::var("DIRECT_UPLOAD_COLLECTION");
        if alias.is_err() {
            println!("Skipping direct upload test");
            return;
        }

        let (api_token, base_url, _) = extract_test_env();
        let client = BaseClient::new(&base_url, Some(&api_token))
            .expect("Failed to create client");

        // Create a test dataset
        let (_, pid) = create_test_dataset(&client, &alias.unwrap()).await;

        let size: usize = 1024;
        let response = directupload::get_ticket(
            &client,
            pid.as_str(),
            size.pow(4), // 1TB
        ).await;

        assert!(response.is_ok(), "Failed to get tickets for big file");

        println!("{:?}", response);

        let data = response.unwrap().data.unwrap();
        assert!(!data.urls.is_empty(), "URLs should not be empty for big files");
        assert!(data.url.is_none(), "URL should be None for big files");
    }

    #[tokio::test]
    async fn test_direct_upload_not_supported() {
        let (api_token, base_url, _) = extract_test_env();
        let client = BaseClient::new(&base_url, Some(&api_token))
            .expect("Failed to create client");

        // Create a test dataset
        let (_, pid) = create_test_dataset(&client, "Root").await;

        let response = directupload::get_ticket(
            &client,
            pid.as_str(),
            10 * 1024 ^ 4, // 10TB
        ).await;

        let message = serde_json::to_string(&response).unwrap();
        assert!(message.contains("ERROR")); // Ugly, but it works
    }
}