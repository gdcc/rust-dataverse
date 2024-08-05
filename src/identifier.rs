use std::fmt::Display;
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
#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Identifier::Id(id) => id.to_string(),
            Identifier::PersistentId(pid) => pid.to_owned(),
        };
        write!(f, "{}", str)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the conversion of string literals to `Identifier` enum variants.
    ///
    /// This test function verifies the `Identifier` enum's ability to correctly parse string literals
    /// into its variants. It tests with both a numeric string, expected to parse into an `Identifier::Id`,
    /// and a DOI string, expected to parse into an `Identifier::PersistentId`. The function asserts that
    /// the parsing process correctly identifies and converts the string literals into the appropriate
    /// `Identifier` variants.
    ///
    /// # Assertions
    /// - Asserts that a numeric string is correctly parsed as an `Identifier::Id`.
    /// - Asserts that a DOI string is correctly parsed as an `Identifier::PersistentId`.
    #[test]
    fn test_identifier_from_str() {
        let id = "123";
        let pid = "doi:10.5072/FK2/ABC123";

        let id = Identifier::from_str(id).unwrap();
        let pid = Identifier::from_str(pid).unwrap();

        match id {
            Identifier::Id(id) => assert_eq!(id, 123),
            _ => panic!("Expected an id"),
        }

        match pid {
            Identifier::PersistentId(pid) => assert_eq!(pid, "doi:10.5072/FK2/ABC123"),
            _ => panic!("Expected a persistent id"),
        }
    }
}