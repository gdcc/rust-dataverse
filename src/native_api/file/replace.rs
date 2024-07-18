use std::collections::HashMap;

use serde_json;

use crate::{
    callback::CallbackFun,
    client::{BaseClient, evaluate_response},
    native_api::dataset::upload::{UploadBody, UploadResponse},
    request::RequestType,
    response::Response,
};

pub async fn replace_file(
    client: &BaseClient,
    id: &String,
    fpath: &String,
    body: &Option<UploadBody>,
    callbacks: Option<HashMap<String, CallbackFun>>,
) -> Result<Response<UploadResponse>, String> {
    // Endpoint metadata
    let path = format!("api/files/{}/replace", id);

    // Build hash maps and body for the request
    let file = HashMap::from([("file".to_string(), fpath.clone())]);
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
