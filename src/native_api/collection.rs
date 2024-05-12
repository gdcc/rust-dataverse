use crate::{
    client::BaseClient, models::collection::CreateBody, models::collection::CreateResponse,
    response::Response,
};
use serde_json;

pub fn create(
    client: &BaseClient,
    parent: &String,
    body: &CreateBody,
) -> Result<Response<CreateResponse>, String> {
    let body = serde_json::to_string(&body).unwrap();
    let response = client.post(&format!("api/dataverses/{}", parent.as_str()), None, &body);

    match response {
        Ok(response) => Ok(response.json::<Response<CreateResponse>>().unwrap()),
        Err(err) => Err(err.to_string()),
    }
}
