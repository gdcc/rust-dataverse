#[cfg(test)]
mod tests {
    use dataverse::{client::BaseClient, native_api::info::version::get_version, response::Status};

    static BASE_URL: &str = "http://localhost:8080";

    #[test]
    fn test_get_version_success() {
        let client = BaseClient::new(&BASE_URL.to_string(), None).unwrap();
        let version = get_version(&client).unwrap();

        match version.status {
            Status::OK => (),
            Status::ERROR => panic!("Error: {}", version.message.as_ref().unwrap()),
        }

        assert_eq!(version.data.is_some(), true);
        assert_eq!(version.message.is_none(), true);
        assert_eq!(version.request_url.is_none(), true);
        assert_eq!(version.request_method.is_none(), true);
    }

    #[test]
    fn test_get_version_error() {
        let client = BaseClient::new(&BASE_URL.to_string(), None).unwrap();
        let version = get_version(&client);

        match version {
            Ok(_) => panic!("Expected an error"),
            Err(_) => (),
        }
    }
}
