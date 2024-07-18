use std::collections::HashMap;

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

import_types!(schema = "models/file/filemeta.json");

pub async fn upload_file_to_dataset(
    client: &BaseClient,
    id: &Identifier,
    fpath: &String,
    body: &Option<UploadBody>,
    callback: Option<CallbackFun>,
) -> Result<Response<UploadResponse>, String> {
    // Endpoint metadata
    let path = match id {
        Identifier::PersistentId(_) => "api/datasets/:persistentId/add".to_string(),
        Identifier::Id(id) => format!("api/datasets/{}/add", id),
    };

    // Build hash maps for the request
    let file = HashMap::from([("file".to_string(), fpath.clone())]);
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
