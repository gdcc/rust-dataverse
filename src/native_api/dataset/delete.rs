use serde::{Deserialize, Serialize};
use typify::import_types;

use crate::{
    client::{BaseClient, evaluate_response},
    request::RequestType,
    response::Response,
};

import_types!(
    schema = "models/dataset/delete.json",
    struct_builder = true,
);

pub async fn delete_dataset(
    client: &BaseClient,
    id: &i64,
) -> Result<Response<UnpublishedDatasetDeleteResponse>, String> {
    // Endpoint metadata
    let url = format!("/api/datasets/{}", id.to_string());

    // Send request
    let context = RequestType::Plain;
    let response = client.delete(url.as_str(), None, &context).await;

    evaluate_response::<UnpublishedDatasetDeleteResponse>(response).await
}
