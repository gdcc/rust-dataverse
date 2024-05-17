use std::path::PathBuf;

use crate::client::BaseClient;
use crate::native_api::dataset::create::{self, DatasetCreateBody};
use structopt::StructOpt;

use super::base::{evaluate_and_print_response, parse_file, Matcher};

#[derive(StructOpt, Debug)]
#[structopt(about = "Handle datasets of the Dataverse instance")]
pub enum DatasetSubCommand {
    #[structopt(about = "Create a dataset")]
    Create {
        #[structopt(long, short, help = "Alias of the parent dataverse")]
        parent: String,

        #[structopt(
            long,
            short,
            help = "Path to the JSON/YAML file containing the dataset body"
        )]
        body: PathBuf,
    },
}

impl Matcher for DatasetSubCommand {
    fn process(&self, client: &BaseClient) {
        match self {
            DatasetSubCommand::Create { parent, body } => {
                let body: DatasetCreateBody =
                    parse_file::<_, DatasetCreateBody>(body).expect("Failed to parse the file");
                let response = create::create_collection(client, &parent, &body);
                evaluate_and_print_response(response);
            }
        };
    }
}
