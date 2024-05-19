use crate::{
    client::{evaluate_response, BaseClient, RequestType},
    native_api::dataset::edit::GetDatasetResponse,
    response::Response,
};

use std::collections::HashMap;

pub fn get_dataset_meta(
    client: &BaseClient,
    pid: &String,
) -> Result<Response<GetDatasetResponse>, String> {
    // Endpoint metadata
    let url = "/api/datasets/:persistentId";

    // Build Parameters
    let parameters = Some(HashMap::from([(
        "persistentId".to_string(),
        pid.to_owned(),
    )]));

    // Send request
    let context = RequestType::Plain;
    let response = client.get(url, parameters, &context);

    evaluate_response::<GetDatasetResponse>(response)
}
