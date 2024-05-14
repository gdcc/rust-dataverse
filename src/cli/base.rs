use std::error::Error;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::response::Response;
use clap::ArgMatches;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn evaluate_and_print_response<T: Serialize>(response: Result<Response<T>, String>) {
    match response {
        Ok(response) => {
            response.print_result();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn get_argument<U, T>(matches: &ArgMatches, arg_name: &str) -> T
where
    U: AsRef<str> + Send + Sync + Clone + 'static,
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    let value = matches
        .get_one::<U>(arg_name)
        .expect(&format!("{} is required.", arg_name))
        .as_ref()
        .parse::<T>()
        .expect(&format!("{} is invalid.", arg_name));

    value
}

pub fn parse_file<P, T>(path: P) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let content = fs::read_to_string(path)?;

    if let Ok(content) = serde_json::from_str(&content) {
        Ok(content)
    } else if let Ok(content) = serde_yaml::from_str(&content) {
        Ok(content)
    } else {
        Err("Failed to parse the file as either JSON or YAML".into())
    }
}
