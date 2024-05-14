use crate::client::BaseClient;
use crate::native_api;
use clap::{ArgMatches, Command};

use super::base::evaluate_and_print_response;

// CLI commands
pub fn info_subcommand() -> Command {
    Command::new("info")
        .about("Retrieve information about the Dataverse instance")
        .arg_required_else_help(true)
        // Subcommands
        .subcommand(version_subcommand())
}

fn version_subcommand<'a, 'b>() -> Command {
    Command::new("version").about("Get the version of the Dataverse instance")
}

// Execute the appropriate function based on the subcommand
pub fn info_matcher(matches: &ArgMatches, client: &BaseClient) {
    match matches.subcommand() {
        Some(("version", _)) => get_version(client),
        _ => {
            println!("No subcommand");
        }
    }
}

fn get_version(client: &BaseClient) {
    let response = native_api::info::version::get_version(client);
    evaluate_and_print_response(response);
}
