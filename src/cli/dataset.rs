use std::path::PathBuf;

use crate::client::BaseClient;
use crate::native_api::dataset::create::{self, DatasetCreateBody};
use structopt::StructOpt;

use super::base::{evaluate_and_print_response, parse_file, Matcher};

#[derive(StructOpt, Debug)]
#[structopt(about = "Handle datasets of the Dataverse instance")]
pub enum DatasetSubCommand {
    Create {
        #[structopt(long, help = "Alias of the parent dataverse")]
        parent: String,

        #[structopt(long, help = "Path to the JSON file containing the dataset body")]
        path: PathBuf,
    },
}

impl Matcher for DatasetSubCommand {
    fn process(&self, client: &BaseClient) {
        match self {
            DatasetSubCommand::Create { parent, path } => {
                let body: DatasetCreateBody =
                    parse_file::<_, DatasetCreateBody>(path).expect("Failed to parse the file");
                let response = create::create_collection(client, &parent, &body);
                evaluate_and_print_response(response);
            }
        };
    }
}
