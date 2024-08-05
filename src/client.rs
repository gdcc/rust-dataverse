use std::collections::HashMap;

use atty::Stream;
use colored::Colorize;
use reqwest::Client;
use reqwest::Url;
use serde::Deserialize;

use crate::request::RequestType;
use crate::response::Response;

pub struct BaseClient {
    base_url: Url,
    api_token: Option<String>,
    client: Client,
    headers: HashMap<String, String>,
}

// This is the base client that will be used to make requests to the API.
// Its acts as a wrapper around the reqwest::blocking::Client and provides
// methods to make GET, POST, PUT, and DELETE requests.
impl BaseClient {
    pub fn new(
        base_url: &String,
        api_token: Option<&String>,
    ) -> Result<Self, reqwest::Error> {
        let base_url = Url::parse(base_url).unwrap();
        let client = Client::new();
        Ok(BaseClient {
            base_url,
            api_token: api_token.map(|s| s.to_owned().to_string()),
            client,
            headers: HashMap::new(),
        })
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub async fn get(
        &self,
        path: &str,
        parameters: Option<HashMap<String, String>>,
        context: &RequestType,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.perform_request(reqwest::Method::GET, path, parameters, context).await
    }

    pub async fn post(
        &self,
        path: &str,
        parameters: Option<HashMap<String, String>>,
        context: &RequestType,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.perform_request(reqwest::Method::POST, path, parameters, context).await
    }

    pub async fn put(
        &self,
        path: &str,
        parameters: Option<HashMap<String, String>>,
        context: &RequestType,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.perform_request(reqwest::Method::PUT, path, parameters, context).await
    }

    pub async fn delete(
        &self,
        path: &str,
        parameters: Option<HashMap<String, String>>,
        context: &RequestType,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.perform_request(reqwest::Method::DELETE, path, parameters, context).await
    }

    pub async fn patch(
        &self,
        path: &str,
        parameters: Option<HashMap<String, String>>,
        context: &RequestType,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.perform_request(reqwest::Method::PATCH, path, parameters, context).await
    }

    async fn perform_request(
        &self,
        method: reqwest::Method,
        path: &str,
        parameters: Option<HashMap<String, String>>,
        context: &RequestType,
    ) -> Result<reqwest::Response, reqwest::Error> {
        // Process the URL and build the request based on the context
        let url = self.base_url.join(path).unwrap();
        let request = context
            .to_request(self.client.request(method, url.clone())).await;
        let request = match parameters {
            Some(parameters) => request.query(&parameters),
            None => request,
        };

        print_call(url.to_string());

        // Add the API token if it exists
        let request = match &self.api_token {
            Some(api_token) => request.header("X-Dataverse-key", api_token),
            None => request,
        };

        request.send().await
    }
}

// Helper function to evaluate a response
pub async fn evaluate_response<T>(
    response: Result<reqwest::Response, reqwest::Error>,
) -> Result<Response<T>, String>
where
    T: for<'de> Deserialize<'de>,
{
    // Check if the response is an error
    let response = match response {
        Ok(response) => response,
        Err(err) => {
            print_error(err.to_string());
            panic!();
        }
    };

    // Try to read the response into the response struct
    let raw_content = response.text().await.unwrap();
    let json = serde_json::from_str::<Response<T>>(&raw_content);

    match json {
        Ok(json) => Ok(json),
        Err(err) => {
            print_error(
                format!(
                    "{} - {}",
                    err.to_string().red().bold(),
                    raw_content.red().bold(),
                )
                    .to_string(),
            );
            panic!("{}", err.to_string());
        }
    }
}

fn print_error(error: String) {
    println!("\n{} {}\n", "Error:".red().bold(), error, );
}

fn print_call(url: String) {
    if atty::is(Stream::Stdout) {
        println!(
            "{}: {}",
            "Calling".to_string().blue().bold(),
            url
        );
    }
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use lazy_static::lazy_static;
    use serde::Serialize;

    use super::*;

    lazy_static! {
        static ref MOCK_SERVER: MockServer = MockServer::start();
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct ExampleBody {
        key1: String,
        key2: String,
    }

    impl PartialEq for ExampleBody {
        fn eq(&self, other: &Self) -> bool {
            self.key1 == other.key1 && self.key2 == other.key2
        }
    }

    impl Clone for ExampleBody {
        fn clone(&self) -> Self {
            ExampleBody {
                key1: self.key1.clone(),
                key2: self.key2.clone(),
            }
        }
    }

    #[tokio::test]
    async fn test_get_request() {
        let client = BaseClient::new(&MOCK_SERVER.base_url(), None).unwrap();

        let _m = MOCK_SERVER.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/test");
            then.status(200).body("test");
        });

        let response = client
            .get("test", None, &RequestType::Plain)
            .await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_json_body_request() {
        // Arrange
        let client = BaseClient::new(&MOCK_SERVER.base_url(), None).unwrap();
        let expected_body = ExampleBody {
            key1: "value1".to_string(),
            key2: "value2".to_string(),
        };

        let raw_body = serde_json::to_string(&expected_body).unwrap();
        let mock = MOCK_SERVER.mock(|when, then| {
            when.method(httpmock::Method::POST).path("/test_json");
            then.status(200).json_body(raw_body.clone());
        });

        // Act
        let response = client
            .post("test_json", None, &RequestType::JSON { body: raw_body })
            .await;

        // Assert
        assert!(response.is_ok());

        mock.assert();
    }

    #[tokio::test]
    async fn test_multipart_request() {
        let client = BaseClient::new(&MOCK_SERVER.base_url(), None).unwrap();

        let mock = MOCK_SERVER.mock(|when, then| {
            when.method(httpmock::Method::POST).path("/test_multipart");
            then.status(200).body("test");
        });

        // Mock the body
        let expected_body = serde_json::json!({
            "key1": "value1",
            "key2": "value2"
        });

        let context = RequestType::Multipart {
            bodies: Some(HashMap::from([(
                "body".to_string(),
                expected_body.to_string(),
            )])),
            files: Some(HashMap::from([(
                "file".to_string(),
                "tests/fixtures/file.txt".into(),
            )])),
            callbacks: None,
        };

        // Act
        let response = client
            .post("test_multipart", None, &context)
            .await;

        // Assert
        assert!(response.is_ok());

        mock.assert();
    }

    #[tokio::test]
    async fn test_parameter_request() {
        let client = BaseClient::new(&MOCK_SERVER.base_url(), None).unwrap();

        let mock = MOCK_SERVER.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/test_parameters")
                .query_param("key1", "value1")
                .query_param("key2", "value2");
            then.status(200).body("test");
        });

        let parameters = Some(HashMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ]));

        let response = client
            .get("test_parameters", parameters, &RequestType::Plain)
            .await;

        assert!(response.is_ok());

        mock.assert();
    }
}
