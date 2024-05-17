use serde::{Deserialize, Serialize};
use serde_json;
use typify::import_types;

use crate::{
    client::{evaluate_response, BaseClient, RequestType},
    response::Response,
};

import_types!(schema = "models/dataset/create.json");

pub fn create_dataset(
    client: &BaseClient,
    parent: &String,
    body: &DatasetCreateBody,
) -> Result<Response<DatasetCreateResponse>, String> {
    // Endpoint metadata
    let url = format!("api/dataverses/{}/datasets", parent.as_str());

    // Build body
    let body = serde_json::to_string(&body).unwrap();

    // Send request
    let context = RequestType::JSON { body: body.clone() };
    let response = client.post(url.as_str(), None, &context);

    evaluate_response::<DatasetCreateResponse>(response)
}
