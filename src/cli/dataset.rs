use std::path::PathBuf;

use structopt::StructOpt;

use crate::client::BaseClient;
use crate::identifier::Identifier;
use crate::native_api::dataset::create::{self, DatasetCreateBody};
use crate::native_api::dataset::delete;
use crate::native_api::dataset::edit;
use crate::native_api::dataset::edit::EditMetadataBody;
use crate::native_api::dataset::get;
use crate::native_api::dataset::link;
use crate::native_api::dataset::publish::{self, Version};
use crate::native_api::dataset::upload::{self, UploadBody};

use super::base::{evaluate_and_print_response, parse_file, Matcher};

#[derive(StructOpt, Debug)]
#[structopt(about = "Handle datasets of the Dataverse instance")]
pub enum DatasetSubCommand {
    #[structopt(about = "Retrieve a datasets metadata")]
    Get {
        #[structopt(help = "(Peristent) identifier of the dataset to retrieve")]
        id: Identifier,
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
        id: i64,
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

    #[structopt(about = "Link a dataset to another collection")]
    Link {
        #[structopt(long, short, help = "(Persistent) identifier of the dataset to link")]
        id: Identifier,

        #[structopt(long, short, help = "Alias of the collection to link the dataset to")]
        collection: String,
    },

    #[structopt(about = "Upload a file to a dataset")]
    Upload {
        #[structopt(
            long,
            short,
            help = "(Peristent) Identifier of the dataset to upload the file to"
        )]
        id: Identifier,

        #[structopt(help = "Path to the file to upload")]
        path: PathBuf,

        #[structopt(long, help = "Path to the JSON/YAML file containing the file body")]
        body: Option<PathBuf>,
    },
}

impl Matcher for DatasetSubCommand {
    fn process(&self, client: &BaseClient) {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        match self {
            DatasetSubCommand::Get { id } => {
                let response = runtime.block_on(get::get_dataset_meta(client, id));
                evaluate_and_print_response(response);
            }
            DatasetSubCommand::Create { collection, body } => {
                let body: DatasetCreateBody =
                    parse_file::<_, DatasetCreateBody>(body).expect("Failed to parse the file");
                let response = runtime.block_on(create::create_dataset(client, &collection, &body));
                evaluate_and_print_response(response);
            }
            DatasetSubCommand::Publish { pid, version } => {
                let response = runtime.block_on(publish::publish_dataset(client, &pid, version));
                evaluate_and_print_response(response);
            }
            DatasetSubCommand::Delete { id } => {
                let response = runtime.block_on(delete::delete_dataset(client, id));
                evaluate_and_print_response(response);
            }
            DatasetSubCommand::Edit { pid, body, replace } => {
                let body =
                    parse_file::<_, EditMetadataBody>(body).expect("Failed to parse the file");
                let response =
                    runtime.block_on(edit::edit_dataset_metadata(client, &pid, replace, &body));
                evaluate_and_print_response(response);
            }
            DatasetSubCommand::Link { id, collection } => {
                let response = runtime.block_on(link::link_dataset(client, id, collection));
                evaluate_and_print_response(response);
            }
            DatasetSubCommand::Upload { id, path, body } => {
                let body = body.as_ref().map(|body| {
                    parse_file::<_, UploadBody>(body).expect("Failed to parse the file")
                });

                let response = runtime.block_on(upload::upload_file_to_dataset(
                    client,
                    id,
                    &path.to_str().unwrap().to_string(),
                    &body,
                    None,
                ));

                evaluate_and_print_response(response);
            }
        };
    }
}
