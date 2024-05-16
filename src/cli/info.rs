use crate::client::BaseClient;
use crate::native_api;
use structopt::StructOpt;

use super::base::{evaluate_and_print_response, Matcher};

#[derive(StructOpt, Debug)]
#[structopt(about = "Retrieve information about the Dataverse instance")]
pub enum InfoSubCommand {
    #[structopt(about = "Retrieve the version of the Dataverse instance")]
    Version,
}

impl Matcher for InfoSubCommand {
    fn process(&self, client: &BaseClient) {
        let response = match self {
            InfoSubCommand::Version => native_api::info::version::get_version(client),
        };

        evaluate_and_print_response(response);
    }
}
