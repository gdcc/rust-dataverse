use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use crate::directupload::{get_ticket, register, register_file};
use crate::directupload::register::{Checksum, DirectUploadBody, DirectUploadResponse};
use crate::directupload::tickets::process_ticket;
use crate::prelude::{BaseClient, CallbackFun};
use crate::response::Response;

/// Uploads a file to S3 and registers it.
///
/// This asynchronous function performs the following steps:
/// 1. Uploads the file to S3 using the `upload_file_to_s3` function.
/// 2. Registers the uploaded file using the `register_file` function.
///
/// # Arguments
///
/// * `base_client` - A reference to the `BaseClient` used for making API requests.
/// * `filepath` - A `PathBuf` representing the path to the file to be uploaded.
/// * `pid` - A string slice representing the project ID.
/// * `callback_fun` - An optional `CallbackFun` instance for handling callbacks during the upload process.
/// * `body` - A `DirectUploadBody` instance to be updated with the file's details.
///
/// # Returns
///
/// A `Result` wrapping a `Response<DirectUploadResponse>` if the upload and registration are successful, or a `String` error message if they fail.
pub async fn direct_upload(
    base_client: &BaseClient,
    filepath: PathBuf,
    pid: &str,
    callback_fun: Option<CallbackFun>,
    body: register::DirectUploadBody,
) -> Result<Response<DirectUploadResponse>, String> {
    let body = upload_file_to_s3(
        base_client,
        &filepath,
        pid,
        callback_fun,
        body,
    ).await?;

    register_file(
        base_client,
        pid,
        body,
    ).await
}

/// Uploads multiple files to S3 and registers them.
///
/// This asynchronous function performs the following steps:
/// 1. Iterates over the provided file paths and their corresponding callback functions and bodies.
/// 2. Uploads each file to S3 using the `upload_file_to_s3` function.
/// 3. Collects the results of the uploads.
/// 4. Registers the uploaded files using the `register_multiple_files` function.
///
/// # Arguments
///
/// * `base_client` - A reference to the `BaseClient` used for making API requests.
/// * `filepaths` - A vector of `PathBuf` representing the paths to the files to be uploaded.
/// * `pid` - A string slice representing the project ID.
/// * `callback_funs` - An optional vector of `CallbackFun` instances for handling callbacks during the upload process.
/// * `bodies` - A mutable vector of `DirectUploadBody` instances to be updated with the files' details.
///
/// # Returns
///
/// A `Result` wrapping a `Response<DirectUploadResponse>` if the upload and registration are successful, or a `String` error message if they fail.
pub async fn direct_upload_multiple(
    base_client: &BaseClient,
    filepaths: Vec<PathBuf>,
    pid: &str,
    callback_funs: Option<Vec<CallbackFun>>,
    mut bodies: Vec<register::DirectUploadBody>,
) -> Result<Response<DirectUploadResponse>, String> {
    let mut tasks = Vec::new();

    for (idx, filepath) in filepaths.iter().enumerate() {
        let callback_fun = callback_funs.as_ref().and_then(|cbs| cbs.get(idx).cloned());
        let body = bodies
            .get_mut(idx)
            .expect("Index out of bounds for bodies")
            .clone();

        tasks.push(upload_file_to_s3(
            base_client,
            filepath,
            pid,
            callback_fun,
            body,
        ));
    }

    let results = futures::future::join_all(tasks).await;

    // Collect all bodies from the results
    let bodies = results.into_iter()
        .collect::<Result<Vec<DirectUploadBody>, String>>()?;

    register::register_multiple_files(
        base_client,
        pid,
        bodies,
    ).await
}

/// Extracts the file name from a given file path.
///
/// This function takes a reference to a `Path` and attempts to extract the file name
/// as a `String`. It returns `None` if the file name cannot be extracted.
///
/// # Arguments
///
/// * `filepath` - A reference to a `Path` representing the file path.
///
/// # Returns
///
/// An `Option<String>` containing the file name if successful, or `None` if the file name
/// cannot be extracted.
fn get_file_name(filepath: &Path) -> Option<String> {
    filepath
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        .into()
}

/// Uploads a file to S3 and returns the updated `DirectUploadBody`.
///
/// This asynchronous function performs the following steps:
/// 1. Retrieves the file metadata to get its size.
/// 2. Obtains a ticket for the upload.
/// 3. Processes the ticket and uploads the file.
/// 4. Calculates the MD5 checksum of the file.
/// 5. Updates the `DirectUploadBody` with the checksum, file name, and storage identifier.
///
/// # Arguments
///
/// * `base_client` - A reference to the `BaseClient` used for making API requests.
/// * `filepath` - A reference to a `PathBuf` representing the path to the file to be uploaded.
/// * `pid` - A string slice representing the project ID.
/// * `callback_fun` - An optional `CallbackFun` instance for handling callbacks during the upload process.
/// * `body` - A mutable `DirectUploadBody` instance to be updated with the file's details.
///
/// # Returns
///
/// A `Result` wrapping the updated `DirectUploadBody` if the upload is successful, or a `String` error message if it fails.
async fn upload_file_to_s3(
    base_client: &BaseClient,
    filepath: &PathBuf,
    pid: &str,
    callback_fun: Option<CallbackFun>,
    mut body: DirectUploadBody,
) -> Result<DirectUploadBody, String> {
    // Get file meta
    let file_meta = std::fs::metadata(filepath).unwrap();
    let file_size = file_meta.len();

    // Get tickets
    let ticket = get_ticket(
        base_client,
        pid,
        file_size as usize,
    ).await?;

    // Process ticket and upload file
    let storage_identifier = process_ticket(ticket, filepath, callback_fun)
        .await
        .map_err(|e| e.to_string())?;

    // Create the register body and send the request
    let checksum = get_md5_checksum(filepath);
    body.checksum = checksum.into();
    body.file_name = get_file_name(filepath);
    body.storage_identifier = storage_identifier;

    Ok(body)
}

/// Calculates the MD5 checksum of a file.
///
/// This function reads the file incrementally in chunks of 1,000,000 bytes,
/// updates the MD5 hasher with each chunk, and computes the final checksum.
///
/// # Arguments
///
/// * `filepath` - A reference to a `PathBuf` representing the path to the file.
///
/// # Returns
///
/// A `Checksum` struct containing the type ("MD5") and the computed checksum value as a hexadecimal string.
fn get_md5_checksum(filepath: &PathBuf) -> Checksum {
    // Calculate checksum incrementally
    let mut hasher = md5::Context::new();
    let file = std::fs::File::open(filepath).unwrap();
    let buf_size = 1_000_000;
    let mut buffer = BufReader::with_capacity(buf_size, file);

    loop {
        let part = buffer.fill_buf().unwrap();
        if part.is_empty() {
            break;
        }
        hasher.consume(part);
        let part_len = part.len();
        buffer.consume(part_len);
    }

    let digest = hasher.compute();

    Checksum {
        type_: String::from("MD5").into(),
        value: format!("{:x}", digest).into(),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::client::BaseClient;
    use crate::directupload::direct_upload;
    use crate::directupload::register::DirectUploadBody;
    use crate::directupload::uploader::direct_upload_multiple;
    use crate::test_utils::{create_test_dataset, extract_test_env};

    #[tokio::test]
    async fn test_singlepart_upload() {
        // Check if the direct upload env is set
        let alias = std::env::var("DIRECT_UPLOAD_COLLECTION");

        if alias.is_err() {
            println!("Skipping direct upload test");
            return;
        }

        // Set up the client
        let (api_token, base_url, _) = extract_test_env();
        let client = BaseClient::new(&base_url, Some(&api_token)).unwrap();

        // Create a test dataset
        let (_, pid) = create_test_dataset(&client, &alias.unwrap()).await;

        // Create a test file
        let fpath = PathBuf::from("tests/fixtures/file.txt");

        // Create a body
        let body = DirectUploadBody {
            categories: vec!["Data".to_string()],
            checksum: None,
            description: "Some description".to_string().into(),
            directory_label: "some/path".to_string().into(),
            file_name: None,
            mime_type: "text/plain".to_string().into(),
            restrict: None,
            storage_identifier: None,
        };

        // Upload the file
        let response = direct_upload(&client, fpath, &pid, None, body).await;

        assert!(response.is_ok(), "Failed to upload file");
    }

    #[tokio::test]
    async fn test_multiple_singlepart_upload() {
        let alias = std::env::var("DIRECT_UPLOAD_COLLECTION");

        if alias.is_err() {
            println!("Skipping direct upload test");
            return;
        }

        // Set up the client
        let (api_token, base_url, _) = extract_test_env();
        let client = BaseClient::new(&base_url, Some(&api_token)).unwrap();

        // Create a test dataset
        let (_, pid) = create_test_dataset(&client, &alias.unwrap()).await;

        // Create test files
        let files = vec![
            PathBuf::from("tests/fixtures/file.txt"),
            PathBuf::from("tests/fixtures/otherfile.txt"),
        ];

        // Create bodies
        let body1 = DirectUploadBody {
            categories: vec!["Data".to_string()],
            checksum: None,
            description: "Some description".to_string().into(),
            directory_label: "some/path".to_string().into(),
            file_name: None,
            mime_type: "text/plain".to_string().into(),
            restrict: None,
            storage_identifier: None,
        };

        let body2 = DirectUploadBody {
            categories: vec!["Data".to_string()],
            checksum: None,
            description: "Some description".to_string().into(),
            directory_label: "some/path".to_string().into(),
            file_name: None,
            mime_type: "text/plain".to_string().into(),
            restrict: None,
            storage_identifier: None,
        };

        let bodies = vec![body1, body2];

        // Upload the files
        let response = direct_upload_multiple(
            &client,
            files,
            &pid,
            None,
            bodies,
        ).await;

        assert!(response.is_ok(), "Failed to upload files");
    }
}