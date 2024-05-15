use atty::Stream;
use colored::Colorize;
use colored_json::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Status {
    OK,
    ERROR,
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

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[allow(non_snake_case)]
pub struct Response<T> {
    pub status: Status,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestUrl: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestMethod: Option<String>,
}

impl<T> Response<T>
where
    T: serde::Serialize,
{
    pub fn is_ok(&self) -> &Response<T> {
        match self.status {
            Status::ERROR => {
                panic!("Error: {}", self.message.as_ref().unwrap())
            }
            _ => self,
        }
    }

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

    fn redirect_stream(&self, json_str: &str) {
        if atty::is(Stream::Stdout) {
            println!(
                "\n{} - Received the following response: \n",
                "🎉 Success!".green().bold()
            );

            println!("{}\n", json_str.to_colored_json_auto().unwrap());
        } else {
            println!("{}", json_str);
        }
    }
}
