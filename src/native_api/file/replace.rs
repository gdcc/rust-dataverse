use std::collections::HashMap;

use crate::{
    client::{evaluate_response, BaseClient, RequestType},
    native_api::file::upload::{UploadBody, UploadResponse},
    response::Response,
};
use serde_json;

pub fn replace_file(
    client: &BaseClient,
    id: &String,
    fpath: &String,
    body: &Option<UploadBody>,
) -> Result<Response<UploadResponse>, String> {
    // Endpoint metadata
    let path = format!("api/files/{}/replace", id);

    // Build hash maps and body for the request
    let file = HashMap::from([("file".to_string(), fpath.clone())]);
    let body = match body {
        Some(body) => Some(HashMap::from([(
            "jsonData".to_string(),
            serde_json::to_string(&body).unwrap(),
        )])),
        None => None,
    };

    // Send request
    let context = RequestType::Multipart {
        bodies: body,
        files: Some(file),
    };

    let response = client.post(path.as_str(), None, &context);

    evaluate_response::<UploadResponse>(response)
}
