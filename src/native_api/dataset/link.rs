use serde::{Deserialize, Serialize};
use typify::import_types;

use crate::{
    client::{BaseClient, evaluate_response},
    identifier::Identifier,
    request::RequestType,
    response::Response,
    utils::get_dataset_id,
};

import_types!("models/message.json");

pub async fn link_dataset(
    client: &BaseClient,
    id: &Identifier,
    collection_id: &String,
) -> Result<Response<MessageResponse>, String> {
    // Determine dataset id
    let dataset_id = match id {
        Identifier::PersistentId(_) => get_dataset_id(client, id).await?,
        Identifier::Id(id) => id.clone(),
    };

    // Endpoint metadata
    let url = format!("/api/datasets/{}/link/{}", dataset_id, collection_id);

    // Send request
    let context = RequestType::Plain;
    let response = client.put(&url, None, &context).await;

    evaluate_response::<MessageResponse>(response).await
}
