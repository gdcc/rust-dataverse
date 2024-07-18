use std::str::FromStr;

use serde::{Deserialize, Serialize};

// We differentiate between persistent identifiers and
// regular identifiers here. This makes it easier to
// handle the two types of identifiers in the codebase
// without having to check for the presence of a persistent
// identifier every time we need to use an identifier.
//
// This way users can supply a general identifier without specifying
// whether it is a persistent identifier or not. The code will
// automatically determine the type of identifier and use it.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Identifier {
    PersistentId(String),
    Id(i64),
}

impl FromStr for Identifier {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // If it can be parsed as an integer, it is an id
        // Otherwise, it is a persistent id
        match s.parse::<i64>() {
            Ok(_) => Ok(Identifier::Id(s.parse::<i64>().unwrap())),
            Err(_) => Ok(Identifier::PersistentId(s.to_owned())),
        }
    }
}
