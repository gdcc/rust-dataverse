use crate::{
    client::BaseClient, identifier::Identifier, native_api::dataset::get::get_dataset_meta,
};

/// Retrieves the dataset ID for a dataset identified by a persistent identifier (PID).
///
/// This asynchronous function sends a request to retrieve the metadata of a dataset using its PID.
/// It then extracts and returns the dataset ID from the metadata response.
///
/// # Arguments
///
/// * `client` - A reference to the `BaseClient` instance used to send the request.
/// * `pid` - An `Identifier` enum instance representing the unique identifier of the dataset.
///
/// # Returns
///
/// A `Result` wrapping an `i64` representing the dataset ID if the request is successful,
/// or a `String` error message on failure.
pub async fn get_dataset_id(client: &BaseClient, pid: Identifier) -> Result<i64, String> {
    let response = get_dataset_meta(client, pid).await?;
    match response.data {
        Some(data) => Ok(data.id.unwrap()),
        None => Err("No data found".to_string()),
    }
}