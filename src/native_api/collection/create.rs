use serde::{Deserialize, Serialize};
use serde_json;
use typify::import_types;

use crate::{client::BaseClient, response::Response};

import_types!(schema = "models/collection/create.json");

pub fn create_collection(
    client: &BaseClient,
    parent: &String,
    body: &CollectionCreateBody,
) -> Result<Response<CollectionCreateResponse>, String> {
    let body = serde_json::to_string(&body).unwrap();
    let response = client.post(&format!("api/dataverses/{}", parent.as_str()), None, &body);

    match response {
        Ok(response) => Ok(response
            .json::<Response<CollectionCreateResponse>>()
            .unwrap()),
        Err(err) => Err(err.to_string()),
    }
}
