use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct VersionResponse {
    #[serde(deserialize_with = "deserialize_version")]
    pub version: (u32, u32),
    pub build: Option<String>,
}

fn deserialize_version<'de, D>(deserializer: D) -> Result<(u32, u32), D::Error>
where
    D: Deserializer<'de>,
{
    let version_str = String::deserialize(deserializer)?;
    let parts: Vec<u32> = version_str
        .split('.')
        .map(|part| part.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(serde::de::Error::custom)?;

    if parts.len() != 2 {
        return Err(serde::de::Error::custom("Version should have two parts"));
    }

    Ok((parts[0], parts[1]))
}
