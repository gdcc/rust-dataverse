use serde::{Deserialize, Serialize};
use typify::import_types;

use crate::{
    client::{BaseClient, evaluate_response},
    request::RequestType,
    response::Response,
};

import_types!(schema = "models/collection/content.json");

pub async fn get_collection_content(
    client: &BaseClient,
    alias: &String,
) -> Result<Response<Vec<CollectionContent>>, String> {
    // Endpoint metadata
    let url = format!("api/dataverses/{}/contents", alias.as_str());

    // Send request
    let context = RequestType::Plain;
    let response = client.get(url.as_str(), None, &context).await;

    evaluate_response::<Vec<CollectionContent>>(response).await
}
