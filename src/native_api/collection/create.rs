use serde::{Deserialize, Serialize};
use serde_json;
use typify::import_types;

use crate::{
    client::{evaluate_response, BaseClient},
    request::RequestType,
    response::Response,
};

import_types!(schema = "models/collection/create.json");

pub fn create_collection(
    client: &BaseClient,
    parent: &String,
    body: &CollectionCreateBody,
) -> Result<Response<CollectionCreateResponse>, String> {
    let body = serde_json::to_string(&body).unwrap();
    let context = RequestType::JSON { body: body.clone() };
    let response = client.post(
        &format!("api/dataverses/{}", parent.as_str()),
        None,
        &context,
    );

    evaluate_response::<CollectionCreateResponse>(response)
}
