use std::collections::HashMap;
use std::path::PathBuf;

use serde_json;

use crate::{
    callback::CallbackFun,
    client::{BaseClient, evaluate_response},
    native_api::dataset::upload::{UploadBody, UploadResponse},
    request::RequestType,
    response::Response,
};

/// Replaces a file in a dataset identified by a file ID.
///
/// This asynchronous function sends a POST request to the API endpoint designated for replacing files in a dataset.
/// The function constructs the API endpoint URL dynamically, incorporating the file's ID. It sets up the request context
/// for a multipart request, including the file path, optional body metadata, and optional callbacks.
///
/// # Arguments
///
/// * `client` - A reference to the `BaseClient` instance used to send the request.
/// * `id` - A string slice that holds the identifier of the file to be replaced.
/// * `fpath` - A `PathBuf` instance representing the file path of the new file to be uploaded.
/// * `body` - An optional reference to an `UploadBody` struct instance containing additional metadata for the upload.
/// * `callbacks` - An optional `HashMap` of callback functions for handling events during the upload process.
///
/// # Returns
///
/// A `Result` wrapping a `Response<UploadResponse>`, which contains the HTTP response status and the deserialized
/// response data indicating the outcome of the upload operation, if the request is successful, or a `String` error message on failure.
pub async fn replace_file(
    client: &BaseClient,
    id: &str,
    fpath: PathBuf,
    body: &Option<UploadBody>,
    callbacks: Option<HashMap<String, CallbackFun>>,
) -> Result<Response<UploadResponse>, String> {
    // Endpoint metadata
    let path = format!("api/files/{}/replace", id);

    // Build hash maps and body for the request
    let file = HashMap::from([("file".to_string(), fpath)]);
    let body = Option::map(
        body.as_ref(),
        |b| HashMap::from([
            ("jsonData".to_string(), serde_json::to_string(&b).unwrap())
        ]),
    );

    // Send request
    let context = RequestType::Multipart {
        bodies: body,
        files: Some(file),
        callbacks,
    };

    let response = client.post(path.as_str(), None, &context).await;

    evaluate_response::<UploadResponse>(response).await
}
