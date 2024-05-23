use crate::progressbar::wrap_progressbar;
use indicatif::MultiProgress;
use reqwest::blocking::{multipart, RequestBuilder};
use std::collections::HashMap;

// We distinguish between three types of requests: plain, JSON, and multipart
pub enum RequestType {
    // A plain request with no body
    Plain,

    // A JSON request with a JSON body and the
    // content type set to application/json
    JSON {
        body: String,
    },

    // A multipart request with a body and files
    Multipart {
        bodies: Option<HashMap<String, String>>,
        files: Option<HashMap<String, String>>,
    },
}

impl RequestType {
    // Convert the request type to a request builder
    pub fn to_request(&self, request: RequestBuilder) -> RequestBuilder {
        match self {
            RequestType::Plain => request,
            RequestType::JSON { body } => Self::build_json_request(body, request),
            RequestType::Multipart { bodies, files } => {
                Self::build_form_request(bodies, files, request)
            }
        }
    }

    fn build_json_request(body: &str, request: RequestBuilder) -> RequestBuilder {
        request
            .header("Content-Type", "application/json")
            .body(body.to_owned())
    }

    fn build_form_request(
        bodies: &Option<HashMap<String, String>>,
        files: &Option<HashMap<String, String>>,
        request: RequestBuilder,
    ) -> RequestBuilder {
        let mut form = multipart::Form::new();

        if let Some(bodies) = bodies {
            for (key, value) in bodies {
                form = form.part(key.clone(), multipart::Part::text(value.clone()));
            }
        }

        if let Some(files) = files {
            for (key, value) in files {
                let multi_pb = MultiProgress::new();
                let part = wrap_progressbar(value, &multi_pb)
                    .expect("The progress bar could not be created. Please check the file path.");

                form = form.part(key.clone(), part);
            }
        }

        request.multipart(form)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::blocking::Client;
    use std::collections::HashMap;

    #[test]
    fn test_request_type_to_request_plain() {
        // Arrange
        let request = RequestType::Plain
            .to_request(Client::new().request(reqwest::Method::GET, "http://localhost"));

        // Act
        let request = request.build().expect("Could not build request");

        assert_eq!(request.url().as_str(), "http://localhost/");
        assert_eq!(request.method(), reqwest::Method::GET);
    }

    #[test]
    fn test_request_type_to_request_json() {
        // Arrange
        let request = RequestType::JSON {
            body: "{}".to_string(),
        }
        .to_request(Client::new().request(reqwest::Method::GET, "http://localhost"));

        // Act
        let request = request.build().expect("Could not build request");

        // Assert
        assert_eq!(request.url().as_str(), "http://localhost/");
        assert_eq!(request.method(), reqwest::Method::GET);
        assert_eq!(
            request
                .body()
                .expect("Could not get body")
                .as_bytes()
                .expect("Could not get bytes"),
            "{}".as_bytes()
        );
        assert_eq!(
            request.headers().get("Content-Type").unwrap(),
            "application/json"
        );
    }

    #[test]
    fn test_request_type_to_request_form() {
        // Arrange
        let context = RequestType::Multipart {
            bodies: Some(HashMap::from([("body".to_string(), "{}".to_string())])),
            files: Some(HashMap::from([(
                "file".to_string(),
                "tests/fixtures/file.txt".to_string(),
            )])),
        };

        let request =
            context.to_request(Client::new().request(reqwest::Method::GET, "http://localhost"));

        // Act
        let request = request.build().expect("Could not build request");

        // Assert
        assert_eq!(request.url().as_str(), "http://localhost/");
        assert_eq!(request.method(), reqwest::Method::GET);
        assert!(request
            .headers()
            .get("Content-Type")
            .expect("Content-Type not found")
            .to_str()
            .unwrap()
            .contains("multipart/form-data"));
        assert!(
            request.body().is_some(),
            "Body not found in request: {:?}",
            request
        );
    }
}
