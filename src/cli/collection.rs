use std::path::PathBuf;

use structopt::StructOpt;
use tokio::runtime::Runtime;

use crate::client::BaseClient;
use crate::native_api::collection::{content, delete};
use crate::native_api::collection::create::{self, CollectionCreateBody};
use crate::native_api::collection::publish;

use super::base::{evaluate_and_print_response, Matcher, parse_file};

#[derive(StructOpt, Debug)]
#[structopt(about = "Handle collections of a Dataverse instance")]
pub enum CollectionSubCommand {
    #[structopt(about = "Create a collection")]
    Create {
        #[structopt(long, short, help = "Alias of the parent dataverse")]
        parent: String,

        #[structopt(
            long,
            short,
            help = "Path to the JSON/YAML file containing the collection body"
        )]
        body: PathBuf,
    },

    #[structopt(about = "Collection content")]
    Content {
        #[structopt(help = "Alias of the collection")]
        alias: String,
    },

    #[structopt(about = "Publish a collection")]
    Publish {
        #[structopt(help = "Alias of the collection to publish")]
        alias: String,
    },

    #[structopt(about = "Delete a collection")]
    Delete {
        #[structopt(help = "Alias of the collection to delete")]
        alias: String,
    },
}

impl Matcher for CollectionSubCommand {
    fn process(&self, client: &BaseClient) {
        let runtime = Runtime::new().unwrap();
        match self {
            CollectionSubCommand::Content { alias } => {
                let response = 
                    runtime.block_on(content::get_collection_content(client, alias));
                evaluate_and_print_response(response);
            }
            CollectionSubCommand::Create { parent, body } => {
                let body: CollectionCreateBody =
                    parse_file::<_, CollectionCreateBody>(body).expect("Failed to parse the file");
                let response =
                    runtime.block_on(create::create_collection(client, &parent, &body));
                evaluate_and_print_response(response);
            }
            CollectionSubCommand::Publish { alias } => {
                let response =
                    runtime.block_on(publish::publish_collection(client, &alias));
                evaluate_and_print_response(response);
            }
            CollectionSubCommand::Delete { alias } => {
                let response =
                    runtime.block_on(delete::delete_collection(client, &alias));
                evaluate_and_print_response(response);
            }
        };
    }
}
