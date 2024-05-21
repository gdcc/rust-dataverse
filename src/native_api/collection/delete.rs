use serde::{Deserialize, Serialize};
use typify::import_types;

use crate::{
    client::{evaluate_response, BaseClient},
    request::RequestType,
    response::Response,
};

import_types!(schema = "models/collection/delete.json");

pub fn delete_collection(
    client: &BaseClient,
    alias: &String,
) -> Result<Response<CollectionDeleteResponse>, String> {
    // Endpoint metadata
    let url = format!("/api/dataverses/{}", alias.as_str());

    // Send request
    let context = RequestType::Plain;
    let response = client.delete(url.as_str(), None, &context);

    evaluate_response::<CollectionDeleteResponse>(response)
}
