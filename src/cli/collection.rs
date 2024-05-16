use std::path::PathBuf;

use crate::client::BaseClient;
use crate::native_api::collection::create::{self, CollectionCreateBody};
use crate::native_api::collection::publish;
use structopt::StructOpt;

use super::base::{evaluate_and_print_response, parse_file, Matcher};

#[derive(StructOpt, Debug)]
#[structopt(about = "Handle collections of a Dataverse instance")]
pub enum CollectionSubCommand {
    #[structopt(about = "Create a collection")]
    Create {
        #[structopt(long, help = "Alias of the parent dataverse")]
        parent: String,

        #[structopt(long, help = "Path to the JSON file containing the collection body")]
        path: PathBuf,
    },

    #[structopt(about = "Publish a collection")]
    Publish {
        #[structopt(long, help = "Alias of the collection")]
        alias: String,
    },
}

impl Matcher for CollectionSubCommand {
    fn process(&self, client: &BaseClient) {
        match self {
            CollectionSubCommand::Create { parent, path } => {
                let body: CollectionCreateBody =
                    parse_file::<_, CollectionCreateBody>(path).expect("Failed to parse the file");
                let response = create::create_collection(client, &parent, &body);
                evaluate_and_print_response(response);
            }
            CollectionSubCommand::Publish { alias } => {
                let response = publish::publish_collection(client, &alias);
                evaluate_and_print_response(response);
            }
        };
    }
}
