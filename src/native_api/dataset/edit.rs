use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json;
use typify::import_types;

use crate::{
    client::{BaseClient, evaluate_response},
    request::RequestType,
    response::Response,
};

import_types!(
    schema = "models/dataset/edit.json",
    struct_builder = true,
);

pub async fn edit_dataset_metadata(
    client: &BaseClient,
    pid: &String,
    replace: &bool,
    body: &EditMetadataBody,
) -> Result<Response<Dataset>, String> {
    // Endpoint metadata
    let url = "/api/datasets/:persistentId/editMetadata";

    // Build body
    let body = serde_json::to_string(&body).unwrap();

    // Build Parameters
    let parameters = Some(HashMap::from([
        ("persistentId".to_string(), pid.to_owned()),
        ("replace".to_string(), replace.to_string()),
    ]));

    // Send request
    let context = RequestType::JSON { body: body.clone() };
    let response = client.put(url, parameters, &context).await;

    evaluate_response::<Dataset>(response).await
}
