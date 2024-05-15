use serde::{Deserialize, Serialize};
use serde_json;
use typify::import_types;

use crate::{client::BaseClient, response::Response};

import_types!(schema = "models/collection/create.json");

pub fn publish_collection(
    client: &BaseClient,
    alias: &String,
) -> Result<Response<CollectionCreateResponse>, String> {
    let response = client.post(
        &format!("api/dataverses/{}/actions/:publish", alias.as_str()),
        None,
        &"",
    );

    match response {
        Ok(response) => Ok(response
            .json::<Response<CollectionCreateResponse>>()
            .unwrap()),
        Err(err) => Err(err.to_string()),
    }
}
