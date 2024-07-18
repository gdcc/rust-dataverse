use serde::{Deserialize, Serialize};
use serde_json;
use typify::import_types;

use crate::{
    client::{BaseClient, evaluate_response},
    request::RequestType,
    response::Response,
};

import_types!(schema = "models/collection/create.json");

pub async fn create_collection(
    client: &BaseClient,
    parent: &String,
    body: &CollectionCreateBody,
) -> Result<Response<CollectionCreateResponse>, String> {
    // Endpoint metadata
    let url = format!("api/dataverses/{}", parent.as_str());

    // Build body
    let body = serde_json::to_string(&body).unwrap();

    // Send request
    let context = RequestType::JSON { body: body.clone() };
    let response = client.post(url.as_str(), None, &context).await;

    evaluate_response::<CollectionCreateResponse>(response).await
}
