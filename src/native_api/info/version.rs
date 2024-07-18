use regress;
use serde::{Deserialize, Serialize};
use typify::import_types;

use crate::{
    client::{BaseClient, evaluate_response},
    request::RequestType,
    response::Response,
};

import_types!(schema = "models/info/version.json");

pub async fn get_version(client: &BaseClient) -> Result<Response<VersionResponse>, String> {
    let context = RequestType::Plain;
    let response = client.get("api/info/version", None, &context).await;

    evaluate_response::<VersionResponse>(response).await
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use super::*;

    lazy_static! {
        static ref BASE_URL: String = std::env::var("BASE_URL")
            .expect("BASE_URL must be set for tests");
        static ref DV_VERSION: String = std::env::var("DV_VERSION")
            .expect("DV_VERSION must be set for tests");
    }

    #[tokio::test]
    async fn test_get_version() {
        // Arrange
        let client = BaseClient::new(&BASE_URL, None).unwrap();

        // Act
        let response = get_version(&client)
            .await
            .expect("Could not get version");

        // Assert
        assert_eq!(
            response.data.unwrap().version.to_string(),
            *VersionResponseVersion(DV_VERSION.to_string())
        );
    }
}
