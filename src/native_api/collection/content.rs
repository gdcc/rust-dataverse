use serde::{Deserialize, Serialize};
use typify::import_types;

use crate::{
    client::{evaluate_response, BaseClient},
    request::RequestType,
    response::Response,
};

import_types!(schema = "models/collection/content.json");

pub fn get_collection_content(
    client: &BaseClient,
    alias: &String,
) -> Result<Response<Vec<CollectionContent>>, String> {
    // Endpoint metadata
    let url = format!("api/dataverses/{}/contents", alias.as_str());

    // Send request
    let context = RequestType::Plain;
    let response = client.get(url.as_str(), None, &context);

    evaluate_response::<Vec<CollectionContent>>(response)
}
