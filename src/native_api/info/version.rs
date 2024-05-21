use crate::{
    client::{evaluate_response, BaseClient},
    request::RequestType,
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

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref BASE_URL: String = std::env::var("BASE_URL")
            .ok()
            .expect("BASE_URL must be set for tests");
        static ref DV_VERSION: String = std::env::var("DV_VERSION")
            .ok()
            .expect("DV_VERSION must be set for tests");
    }

    #[test]
    fn test_get_version() {
        // Arrange
        let client = BaseClient::new(&BASE_URL.to_string(), None).unwrap();

        // Act
        let response = get_version(&client).expect("Could not get version");

        // Assert
        assert_eq!(
            response.data.unwrap().version.to_string(),
            *VersionResponseVersion(DV_VERSION.to_string())
        );
    }
}
