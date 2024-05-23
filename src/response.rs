use atty::Stream;
use colored::Colorize;
use colored_json::prelude::*;

// We distinguish success and error responses with this enum
// Once the response is parsed, we can check if it's an error or not
// and act accordingly
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Status {
    OK,
    ERROR,
}

impl PartialEq for Status {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Status::OK, Status::OK) => true,
            (Status::ERROR, Status::ERROR) => true,
            _ => false,
        }
    }
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Status::OK => "OK",
            Status::ERROR => "ERROR",
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            Status::OK => true,
            Status::ERROR => false,
        }
    }
}

// This struct acts as a wrapper for the response and
// models the response we expect from Dataverse
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[allow(non_snake_case)]
pub struct Response<T> {
    pub status: Status,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestUrl: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestMethod: Option<String>,
}

impl<T> Response<T>
where
    T: serde::Serialize,
{
    pub fn print_result(&self) {
        match self.status {
            Status::OK => {
                let json = serde_json::to_string_pretty(&self.data.as_ref().unwrap()).unwrap();

                self.redirect_stream(&json);
                std::process::exit(exitcode::OK);
            }
            Status::ERROR => {
                println!(
                    "\n{} {}\n",
                    "Error:".red().bold(),
                    self.message.as_ref().unwrap()
                );
                std::process::exit(exitcode::DATAERR);
            }
        }
    }

    // This function is used to redirect the output to the appropriate stream
    // If users are redirecting the output to a file, we don't want to print
    // the success message but only the JSON response to ensure that the output
    // is clean and can be used in other scripts
    fn redirect_stream(&self, json_str: &str) {
        if atty::is(Stream::Stdout) {
            println!("{}", success_message());
            println!("{}\n", json_str.to_colored_json_auto().unwrap());
        } else {
            println!("{}", json_str);
        }
    }
}

fn success_message() -> String {
    format!(
        "{} {} - Received the following response: \n",
        "└── ".bold(),
        "🎉 Success!".green().bold()
    )
}

// This is a workaround to tackle the issue of having a nested message
// in the response currently caused by the editMetadata endpoint
//
// For more info:
// https://dataverse.zulipchat.com/#narrow/stream/378866-troubleshooting/topic/.E2.9C.94.20Duplicate.20file.20response
//
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Message {
    PlainMessage(String),
    NestedMessage(NestedMessage),
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Message::PlainMessage(message) => write!(f, "{}", message),
            Message::NestedMessage(nested_message) => {
                write!(f, "{}", nested_message.message.as_ref().unwrap())
            }
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct NestedMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}
