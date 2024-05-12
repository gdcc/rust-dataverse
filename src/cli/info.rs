use super::base::SubCommandTrait;
use crate::{client::BaseClient, native_api};
use clap::{Args, Subcommand};
use colored::Colorize;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct InfoArgs {
    #[command(subcommand)]
    pub command: Option<InfoCommands>,
}

#[derive(Debug, Subcommand)]
pub enum InfoCommands {
    #[command(about = "Get the version of the Dataverse instance")]
    Version,
}

impl SubCommandTrait for InfoCommands {
    fn process(&self, client: &BaseClient) {
        match self {
            InfoCommands::Version => InfoCommands::version(client),
        }
    }
}

impl InfoCommands {
    fn version(client: &BaseClient) {
        let response = native_api::info::get_version(client);

        match response {
            Ok(response) => {
                let (major, minor) = response.data.unwrap().version;
                println!(
                    "Dataverse Version: {}.{}",
                    major.to_string().bold(),
                    minor.to_string().bold()
                )
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
