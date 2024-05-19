use crate::{
    client::{evaluate_response, BaseClient, RequestType},
    response::Response,
};
use serde::{Deserialize, Serialize};
use typify::import_types;

import_types!(schema = "models/dataset/delete.json");

pub fn delete_dataset(
    client: &BaseClient,
    id: &u32,
) -> Result<Response<UnpublishedDatasetDeleteResponse>, String> {
    // Endpoint metadata
    let url = format!("/api/datasets/{}", id.to_string());

    // Send request
    let context = RequestType::Plain;
    let response = client.delete(url.as_str(), None, &context);

    evaluate_response::<UnpublishedDatasetDeleteResponse>(response)
}
