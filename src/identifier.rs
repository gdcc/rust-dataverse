use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Identifier {
    PeristentId(String),
    Id(i64),
}

impl FromStr for Identifier {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("doi:") {
            Ok(Identifier::PeristentId(s.to_owned()))
        } else {
            match s.parse::<i64>() {
                Ok(id) => Ok(Identifier::Id(id)),
                Err(_) => Err(format!("Invalid identifier: {}", s)),
            }
        }
    }
}

impl Identifier {
    pub fn from_pid_or_id(pid: &Option<String>, id: &Option<i64>) -> Self {
        if let Some(pid) = pid {
            Identifier::PeristentId(pid.to_owned())
        } else if let Some(id) = id {
            Identifier::Id(id.to_owned())
        } else {
            panic!("Either a persistent identifier or an identifier must be provided")
        }
    }
}
