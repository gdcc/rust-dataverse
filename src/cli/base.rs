use std::str::FromStr;

use crate::response::Response;
use clap::ArgMatches;
use serde::Serialize;

pub fn evaluate_and_print_response<T: Serialize>(response: Result<Response<T>, String>) {
    match response {
        Ok(response) => {
            let json = serde_json::to_string_pretty(&response).unwrap();
            println!("{}", json);
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
