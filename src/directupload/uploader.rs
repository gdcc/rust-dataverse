use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use crate::directupload::{get_ticket, register, register_file};
use crate::directupload::register::{Checksum, DirectUploadResponse};
use crate::directupload::tickets::process_ticket;
use crate::prelude::{BaseClient, CallbackFun};
use crate::response::Response;

pub async fn direct_upload(
    base_client: &BaseClient,
    filepath: PathBuf,
    pid: &str,
    callback_fun: Option<CallbackFun>,
    mut body: register::DirectUploadBody,
) -> Result<Response<DirectUploadResponse>, String> {
    // Get file meta
    let file_meta = std::fs::metadata(&filepath).unwrap();
    let file_size = file_meta.len();

    // Get tickets
    let ticket = get_ticket(
        base_client,
        pid,
        file_size as usize,
    ).await?;

    // Process ticket and upload file
    let storage_identifier = process_ticket(ticket, &filepath, callback_fun)
        .await
        .map_err(|e| e.to_string())?;

    // Create the register body and send the request
    let checksum = get_md5_checksum(&filepath);
    body.checksum = checksum.into();
    body.file_name = get_file_name(&filepath);
    body.storage_identifier = storage_identifier;

    register_file(
        base_client,
        pid,
        body,
    ).await
}

fn get_file_name(filepath: &Path) -> Option<String> {
    filepath
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        .into()
}

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
}