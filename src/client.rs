use std::collections::HashMap;

use reqwest::blocking::Client;
use reqwest::Url;

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
        parameters: Option<&HashMap<String, String>>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.perform_request(reqwest::Method::GET, path, parameters, None)
    }

    pub fn post(
        &self,
        path: &str,
        parameters: Option<&HashMap<String, String>>,
        body: &str,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.perform_request(reqwest::Method::POST, path, parameters, Some(body))
    }

    pub fn put(
        &self,
        path: &str,
        parameters: Option<&HashMap<String, String>>,
        body: &str,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.perform_request(reqwest::Method::PUT, path, parameters, Some(body))
    }

    pub fn delete(
        &self,
        path: &str,
        parameters: Option<&HashMap<String, String>>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.perform_request(reqwest::Method::DELETE, path, parameters, None)
    }

    pub fn patch(
        &self,
        path: &str,
        body: &str,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.perform_request(reqwest::Method::PATCH, path, None, Some(body))
    }

    fn perform_request(
        &self,
        method: reqwest::Method,
        path: &str,
        parameters: Option<&HashMap<String, String>>,
        body: Option<&str>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let url = self.base_url.join(path).unwrap();
        let request = self.client.request(method, url);

        let request = match body {
            Some(body) => request.body(body.to_owned()),
            None => request,
        };

        let request = match parameters {
            Some(parameters) => request.query(&parameters),
            None => request,
        };

        let request = match &self.api_token {
            Some(api_token) => request.header("X-Dataverse-key", api_token),
            None => request,
        };

        let response = request.send();

        response
    }
}
