use super::base::{evaluate_and_print_response, parse_file, Matcher};
use crate::client::BaseClient;
use crate::native_api::dataset::create::{self, DatasetCreateBody};
use crate::native_api::dataset::delete;
use crate::native_api::dataset::edit;
use crate::native_api::dataset::edit::EditMetadataBody;
use crate::native_api::dataset::get;
use crate::native_api::dataset::publish::{self, Version};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Handle datasets of the Dataverse instance")]
pub enum DatasetSubCommand {
    #[structopt(about = "Retrieve a datasets metadata")]
    Get {
        #[structopt(help = "Peristent identifier of the dataset to retrieve")]
        pid: String,
    },

    #[structopt(about = "Create a dataset")]
    Create {
        #[structopt(long, short, help = "Alias of the collection to create the dataset in")]
        collection: String,

        #[structopt(
            long,
            short,
            help = "Path to the JSON/YAML file containing the dataset body"
        )]
        body: PathBuf,
    },

    #[structopt(about = "Publishes a dataset")]
    Publish {
        #[structopt(help = "Persistent identifier of the dataset to publish")]
        pid: String,

        #[structopt(
            long,
            short,
            help = "Version of the dataset to publish (major, minor, updatecurrent)",
            default_value = "major"
        )]
        version: Version,
    },

    #[structopt(about = "Deletes a dataset")]
    Delete {
        #[structopt(help = "Identifier of the dataset to delete")]
        id: u32,
    },

    #[structopt(about = "Edit the metadata of a dataset")]
    Edit {
        #[structopt(long, short, help = "Perisistent identifier of the dataset to edit")]
        pid: String,

        #[structopt(
            long,
            short,
            help = "Path to the JSON/YAML file containing the metadata to edit"
        )]
        body: PathBuf,

        #[structopt(long, short, help = "Whether to replace the metadata or not")]
        replace: bool,
    },
}

impl Matcher for DatasetSubCommand {
    fn process(&self, client: &BaseClient) {
        match self {
            DatasetSubCommand::Get { pid } => {
                let response = get::get_dataset_meta(client, pid);
                evaluate_and_print_response(response);
            }
            DatasetSubCommand::Create { collection, body } => {
                let body: DatasetCreateBody =
                    parse_file::<_, DatasetCreateBody>(body).expect("Failed to parse the file");
                let response = create::create_dataset(client, &collection, &body);
                evaluate_and_print_response(response);
            }
            DatasetSubCommand::Publish { pid, version } => {
                let response = publish::publish_dataset(client, &pid, version);
                evaluate_and_print_response(response);
            }
            DatasetSubCommand::Delete { id } => {
                let response = delete::delete_dataset(client, id);
                evaluate_and_print_response(response);
            }
            DatasetSubCommand::Edit { pid, body, replace } => {
                let body =
                    parse_file::<_, EditMetadataBody>(body).expect("Failed to parse the file");
                let response = edit::edit_dataset_metadata(client, &pid, replace, &body);
                evaluate_and_print_response(response);
            }
        };
    }
}
