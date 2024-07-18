use serde::{Deserialize, Serialize};
use serde_json;
use typify::import_types;

use crate::{
    client::{BaseClient, evaluate_response},
    request::RequestType,
    response::Response,
};

import_types!(schema = "models/collection/create.json");

pub async fn publish_collection(
    client: &BaseClient,
    alias: &String,
) -> Result<Response<CollectionCreateResponse>, String> {
    // Endpoint metadata
    let url = format!("api/dataverses/{}/actions/:publish", alias.as_str());

    // Send request
    let context = RequestType::Plain;
    let response = client.post(url.as_str(), None, &context).await;

    evaluate_response::<CollectionCreateResponse>(response).await
}
