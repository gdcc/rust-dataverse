use serde::{Deserialize, Serialize};
use serde_json;
use typify::import_types;

use crate::{
    client::{evaluate_response, BaseClient, RequestType},
    response::Response,
};

import_types!(schema = "models/collection/create.json");

pub fn publish_collection(
    client: &BaseClient,
    alias: &String,
) -> Result<Response<CollectionCreateResponse>, String> {
    let context = RequestType::Plain;
    let response = client.post(
        &format!("api/dataverses/{}/actions/:publish", alias.as_str()),
        None,
        &context,
    );

    evaluate_response::<CollectionCreateResponse>(response)
}
