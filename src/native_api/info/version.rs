use crate::{client::BaseClient, response::Response};
use regress;
use serde::{Deserialize, Serialize};
use typify::import_types;

import_types!(schema = "models/info/version.json");

pub fn get_version(client: &BaseClient) -> Result<Response<VersionResponse>, String> {
    let response = client.get("api/info/version", None);
    match response {
        Ok(response) => Ok(response.json::<Response<VersionResponse>>().unwrap()),
        Err(err) => Err(err.to_string()),
    }
}
