use crate::{
    client::{evaluate_response, BaseClient},
    identifier::Identifier,
    native_api::dataset::edit::GetDatasetResponse,
    request::RequestType,
    response::Response,
};

use std::collections::HashMap;

pub fn get_dataset_meta(
    client: &BaseClient,
    id: &Identifier,
) -> Result<Response<GetDatasetResponse>, String> {
    // Endpoint metadata
    let url = match id {
        Identifier::PeristentId(_) => "api/datasets/:persistentId".to_string(),
        Identifier::Id(id) => format!("api/datasets/{}", id),
    };

    // Build Parameters
    let parameters = match id {
        Identifier::PeristentId(id) => {
            Some(HashMap::from([("persistentId".to_string(), id.clone())]))
        }
        Identifier::Id(_) => None,
    };

    // Send request
    let context = RequestType::Plain;
    let response = client.get(url.as_str(), parameters, &context);

    evaluate_response::<GetDatasetResponse>(response)
}
