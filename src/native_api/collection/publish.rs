use serde::{Deserialize, Serialize};
use serde_json;
use typify::import_types;

use crate::{
    client::{evaluate_response, BaseClient},
    request::RequestType,
    response::Response,
};

import_types!(schema = "models/collection/create.json");

pub fn publish_collection(
    client: &BaseClient,
    alias: &String,
) -> Result<Response<CollectionCreateResponse>, String> {
    // Endpoint metadata
    let url = format!("api/dataverses/{}/actions/:publish", alias.as_str());

    // Send request
    let context = RequestType::Plain;
    let response = client.post(url.as_str(), None, &context);

    evaluate_response::<CollectionCreateResponse>(response)
}
