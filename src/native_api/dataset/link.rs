use crate::{
    client::{evaluate_response, BaseClient},
    identifier::Identifier,
    request::RequestType,
    response::Response,
    utils::get_dataset_id,
};
use serde::{Deserialize, Serialize};
use typify::import_types;

import_types!("models/message.json");

pub fn link_dataset(
    client: &BaseClient,
    id: &Identifier,
    collection_id: &String,
) -> Result<Response<MessageResponse>, String> {
    // Determine dataset id
    let dataset_id = match id {
        Identifier::PeristentId(_) => get_dataset_id(client, id)?,
        Identifier::Id(id) => id.clone(),
    };

    // Endpoint metadata
    let url = format!("/api/datasets/{}/link/{}", dataset_id, collection_id);

    // Send request
    let context = RequestType::Plain;
    let response = client.put(&url, None, &context);

    evaluate_response::<MessageResponse>(response)
}
