use crate::{client::BaseClient, models::info::VersionResponse, response::Response};

pub fn get_version(client: &BaseClient) -> Result<Response<VersionResponse>, String> {
    let response = client.get("api/info/version", None);
    match response {
        Ok(response) => Ok(response.json::<Response<VersionResponse>>().unwrap()),
        Err(err) => Err(err.to_string()),
    }
}
