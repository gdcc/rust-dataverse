use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use serde_json;
use typify::import_types;

use crate::{
    client::{evaluate_response, BaseClient},
    request::RequestType,
    response::Response,
};

import_types!(schema = "models/dataset/edit.json");

pub fn edit_dataset_metadata(
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
    let response = client.put(url, parameters, &context);

    evaluate_response::<Dataset>(response)
}
