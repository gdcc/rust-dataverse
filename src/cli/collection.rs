use crate::client::BaseClient;
use crate::native_api;
use crate::native_api::collection::create::CollectionCreateBody;
use clap::{arg, ArgMatches, Command};

use super::base::get_argument;

//
// CLI commands
//
pub fn collection_subcommand() -> Command {
    Command::new("collection")
        .about("Handle collections in the Dataverse instance")
        .arg_required_else_help(true)
        // Subcommands
        .subcommand(create_subcommand())
}

fn create_subcommand<'a, 'b>() -> Command {
    Command::new("create")
        .about("Create an empty collection")
        .arg_required_else_help(true)
        .args(&[
            arg!(--parent <NAME> "The parent dataverse alias"),
            arg!(--file <FILE> "The file containing the collection metadata"),
        ])
}

//
// Execute the appropriate function based on the subcommand
//
pub fn collection_matcher(matches: &ArgMatches, client: &BaseClient) {
    match matches.subcommand() {
        Some(("create", _)) => create_collection(matches, client),
        _ => {
            println!("No subcommand");
        }
    }
}

fn create_collection(matches: &ArgMatches, client: &BaseClient) {
    // Extract the arguments
    let parent = get_argument::<String, String>(matches, "parent");
    let path = get_argument::<String, String>(matches, "file");

    // Load the collection metadata from the yaml file
    let body = std::fs::read_to_string(path).expect("Failed to read the file");
    let body: CollectionCreateBody = serde_yaml::from_str(&body).expect("Failed to parse the file");

    let response = native_api::collection::create::create_collection(client, &parent, &body);
    super::base::evaluate_and_print_response(response);
}
