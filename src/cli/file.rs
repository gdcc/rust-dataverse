use std::path::PathBuf;
use structopt::StructOpt;

use crate::client::BaseClient;
use crate::identifier::Identifier;
use crate::native_api::file::replace;
use crate::native_api::file::upload;
use crate::native_api::file::upload::UploadBody;

use super::base::{evaluate_and_print_response, parse_file, Matcher};

#[derive(StructOpt, Debug)]
#[structopt(about = "Handle collections of a Dataverse instance")]
pub enum FileSubCommand {
    #[structopt(about = "Upload a file")]
    Upload {
        #[structopt(help = "Path to the file to upload")]
        path: PathBuf,

        #[structopt(
            long,
            help = "Persistent identifier of the dataset to upload the file to",
            conflicts_with = "id"
        )]
        pid: Option<String>,

        #[structopt(
            long,
            help = "Identifier of the dataset to upload the file to",
            conflicts_with = "pid"
        )]
        id: Option<String>,

        #[structopt(long, help = "Path to the JSON/YAML file containing the file body")]
        body: Option<PathBuf>,
    },

    #[structopt(about = "Replace a file")]
    Replace {
        #[structopt(help = "Path to the file to replace")]
        path: PathBuf,

        #[structopt(long, short, help = "Identifier of the of the file to replace")]
        id: String,

        #[structopt(
            long,
            short,
            help = "Path to the JSON/YAML file containing the file body"
        )]
        body: PathBuf,

        #[structopt(long, short, help = "Force the replacement of the file")]
        force: bool,
    },
}

impl Matcher for FileSubCommand {
    fn process(&self, client: &BaseClient) {
        match self {
            FileSubCommand::Upload {
                pid,
                id,
                path,
                body,
            } => {
                let id = Identifier::from_pid_or_id(pid, id);
                let body = prepare_body(body);
                let response =
                    upload::upload_file(client, &id, &path.to_str().unwrap().to_string(), &body);

                evaluate_and_print_response(response);
            }
            FileSubCommand::Replace {
                id,
                path,
                body,
                force,
            } => {
                let mut body = prepare_body(&Some(body.to_owned())).unwrap();

                if body.force_replace.is_none() {
                    body.force_replace = Some(*force);
                }

                let response = replace::replace_file(
                    client,
                    id,
                    &path.to_str().unwrap().to_string(),
                    &Some(body),
                );

                evaluate_and_print_response(response);
            }
        };
    }
}

fn prepare_body(body: &Option<PathBuf>) -> Option<UploadBody> {
    match body {
        Some(body) => {
            Some(parse_file::<_, UploadBody>(body).expect("Unable to pase the metadata file."))
        }
        _ => None,
    }
}
