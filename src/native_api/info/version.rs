use crate::{
    client::{evaluate_response, BaseClient, RequestType},
    response::Response,
};
use regress;
use serde::{Deserialize, Serialize};
use typify::import_types;

import_types!(schema = "models/info/version.json");

pub fn get_version(client: &BaseClient) -> Result<Response<VersionResponse>, String> {
    let context = RequestType::Plain;
    let response = client.get("api/info/version", None, &context);

    evaluate_response::<VersionResponse>(response)
}
