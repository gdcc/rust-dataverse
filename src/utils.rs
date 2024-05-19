use crate::{
    client::BaseClient, identifier::Identifier, native_api::dataset::get::get_dataset_meta,
};

pub fn get_dataset_id(client: &BaseClient, pid: &Identifier) -> Result<i64, String> {
    let response = get_dataset_meta(client, pid)?;
    match response.data {
        Some(data) => Ok(data.id.unwrap()),
        None => Err("No data found".to_string()),
    }
}
