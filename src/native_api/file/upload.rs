use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json;
use typify::import_types;

use crate::{
    client::{evaluate_response, BaseClient, RequestType},
    identifier::Identifier,
    response::Response,
};

import_types!(schema = "models/file/filemeta.json");

pub fn upload_file(
    client: &BaseClient,
    id: &Identifier,
    fpath: &String,
    body: &Option<UploadBody>,
) -> Result<Response<UploadResponse>, String> {
    // Endpoint metadata
    let path = match id {
        Identifier::PeristentId(_) => "api/datasets/:persistentId/add".to_string(),
        Identifier::Id(id) => format!("api/datasets/{}/add", id),
    };

    // Build hash maps for the request
    let file = HashMap::from([("file".to_string(), fpath.clone())]);

    let body = match body {
        Some(body) => Some(HashMap::from([(
            "jsonData".to_string(),
            serde_json::to_string(&body).unwrap(),
        )])),
        None => None,
    };

    // Build the request context
    let context = RequestType::Multipart {
        bodies: body,
        files: Some(file),
    };

    let response = match id {
        Identifier::PeristentId(id) => client.post(
            path.as_str(),
            Some(HashMap::from([("persistentId".to_string(), id.clone())])),
            &context,
        ),
        Identifier::Id(id) => client.post(&format!("api/files/{}", id), None, &context),
    };

    evaluate_response::<UploadResponse>(response)
}
