use atty::Stream;
use colored::Colorize;
use indicatif::MultiProgress;
use reqwest::blocking::{multipart, Client, RequestBuilder};
use reqwest::Url;
use serde::Deserialize;
use std::collections::HashMap;

use crate::progressbar::wrap_progressbar;
use crate::response::Response;

pub enum RequestType {
    Plain,
    JSON {
        body: String,
    },
    Multipart {
        bodies: Option<HashMap<String, String>>,
        files: Option<HashMap<String, String>>,
    },
}

impl RequestType {
    // Convert the request type to a request
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

pub struct BaseClient {
    base_url: Url,
    api_token: Option<String>,
    client: Client,
}

impl BaseClient {
    pub fn new(base_url: &String, api_token: Option<&String>) -> Result<Self, reqwest::Error> {
        let base_url = Url::parse(base_url).unwrap();
        let client = Client::new();
        Ok(BaseClient {
            base_url,
            api_token: api_token.map(|s| s.to_owned().to_string()),
            client,
        })
    }

    pub fn get(
        &self,
        path: &str,
        parameters: Option<HashMap<String, String>>,
        context: &RequestType,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.perform_request(reqwest::Method::GET, path, parameters, context)
    }

    pub fn post(
        &self,
        path: &str,
        parameters: Option<HashMap<String, String>>,
        context: &RequestType,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.perform_request(reqwest::Method::POST, path, parameters, context)
    }

    pub fn put(
        &self,
        path: &str,
        parameters: Option<HashMap<String, String>>,
        context: &RequestType,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.perform_request(reqwest::Method::PUT, path, parameters, context)
    }

    pub fn delete(
        &self,
        path: &str,
        parameters: Option<HashMap<String, String>>,
        context: &RequestType,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.perform_request(reqwest::Method::DELETE, path, parameters, context)
    }

    pub fn patch(
        &self,
        path: &str,
        parameters: Option<HashMap<String, String>>,
        context: &RequestType,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.perform_request(reqwest::Method::PATCH, path, parameters, context)
    }

    fn perform_request(
        &self,
        method: reqwest::Method,
        path: &str,
        parameters: Option<HashMap<String, String>>,
        context: &RequestType,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let url = self.base_url.join(path).unwrap();
        let request = context.to_request(self.client.request(method, url.clone()));

        let request = match parameters {
            Some(parameters) => request.query(&parameters),
            None => request,
        };

        print_call(url.to_string());

        let request = match &self.api_token {
            Some(api_token) => request.header("X-Dataverse-key", api_token),
            None => request,
        };

        let response = request.send();

        response
    }
}

// Helper function to evaluate a response
pub fn evaluate_response<T>(
    response: Result<reqwest::blocking::Response, reqwest::Error>,
) -> Result<Response<T>, String>
where
    T: for<'de> Deserialize<'de>,
{
    // Check if the response is an error
    let response = match response {
        Ok(response) => response,
        Err(err) => {
            print_error::<T>(err.to_string());
            panic!();
        }
    };

    // Try to read the response into the response struct
    let raw_content = response.text().unwrap();
    let json = serde_json::from_str::<Response<T>>(&raw_content);

    return match json {
        Ok(json) => Ok(json),
        Err(err) => {
            print_error::<T>(
                format!(
                    "{} - {}",
                    err.to_string().red().bold(),
                    raw_content.red().bold(),
                )
                .to_string(),
            );
            panic!();
        }
    };
}

fn print_error<T>(error: String) {
    println!("\n{} {}\n", "Error:".red().bold(), error,);
}

fn print_call(url: String) {
    if atty::is(Stream::Stdout) {
        println!(
            "{}: {}",
            "Calling".to_string().blue().bold(),
            url.to_string()
        );
    }
}
