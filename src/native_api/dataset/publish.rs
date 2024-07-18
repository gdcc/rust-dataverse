use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};
use typify::import_types;

use crate::{
    client::{BaseClient, evaluate_response},
    request::RequestType,
    response::Response,
};

import_types!(schema = "models/dataset/publish.json");

#[derive(Serialize, Deserialize, Debug)]
pub enum Version {
    #[serde(rename = "major")]
    Major,

    #[serde(rename = "minor")]
    Minor,

    #[serde(rename = "updatecurrent")]
    UpdateCurrent,
}

impl FromStr for Version {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "major" => Ok(Version::Major),
            "minor" => Ok(Version::Minor),
            "updatecurrent" => Ok(Version::UpdateCurrent),
            _ => Err(format!("Invalid version: {}", s)),
        }
    }
}

pub async fn publish_dataset(
    client: &BaseClient,
    pid: &String,
    version: &Version,
) -> Result<Response<DatasetPublishResponse>, String> {
    // Endpoint metadata
    let url = "/api/datasets/:persistentId/actions/:publish";

    // Determine version
    let version = match version {
        Version::Major => "major".to_string(),
        Version::Minor => "minor".to_string(),
        Version::UpdateCurrent => "updateCurrent".to_string(),
    };

    // Build request parameters
    let parameters = Some(HashMap::from([
        ("persistentId".to_string(), pid.to_owned()),
        ("type".to_string(), version.to_owned()),
    ]));

    // Send request
    let context = RequestType::Plain;
    let response = client.post(url, parameters, &context).await;

    evaluate_response::<DatasetPublishResponse>(response).await
}
