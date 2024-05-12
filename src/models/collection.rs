use std::fs::File;

use serde::{Deserialize, Serialize};

// Collection creation
//
// This model implements the schema for creating a new collection in Dataverse.
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CreateBody {
    pub name: String,
    pub alias: String,
    pub affiliation: String,
    pub description: String,
    pub dataverseContacts: Vec<DataverseContact>,
    pub dataverseType: DataverseType,
}

impl CreateBody {
    pub fn new(
        name: &str,
        alias: &str,
        affiliation: &str,
        description: &str,
        dataverse_type: DataverseType,
    ) -> Self {
        CreateBody {
            name: name.to_string(),
            alias: alias.to_string(),
            dataverseContacts: Vec::new(),
            affiliation: affiliation.to_string(),
            description: description.to_string(),
            dataverseType: dataverse_type,
        }
    }

    pub fn add_contact(&mut self, contact_email: &str) {
        self.dataverseContacts.push(DataverseContact {
            contactEmail: contact_email.to_string(),
            displayOrder: None,
        });
    }

    pub fn from_yaml(path: &str) -> Result<Self, serde_yaml::Error> {
        let file = File::open(path).unwrap();
        serde_yaml::from_reader(file)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CreateResponse {
    id: u32,
    alias: String,
    name: String,
    affiliation: String,
    dataverseContacts: Vec<DataverseContact>,
    permissionRoot: bool,
    description: String,
    dataverseType: DataverseType,
    ownerId: u32,
    creationDate: String,
    isReleased: bool,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct DataverseContact {
    #[serde(rename = "contactEmail")]
    pub contactEmail: String,

    #[serde(rename = "displayOrder")]
    pub displayOrder: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum DataverseType {
    DEPARTMENT,
    JOURNALS,
    LABORATORY,
    ORGANIZATIONS_INSTITUTIONS,
    RESEARCHERS,
    RESEARCH_GROUP,
    RESEARCH_PROJECTS,
    TEACHING_COURSES,
    UNCATEGORIZED,
}
