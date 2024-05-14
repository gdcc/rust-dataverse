use clap::command;
use dataverse::cli::{collection, info};
use dataverse::client::BaseClient;

fn main() {
    let (base_url, api_token) = extract_config_from_env()
        .expect("Please set the DVCLI_URL and DVCLI_TOKEN environment variables.");

    let client = BaseClient::new(&base_url, api_token.as_ref()).expect("Failed to create client.");

    let matches = command!()
        .about("Control Dataverse from your terminal.")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        // Subcommands
        .subcommand(info::info_subcommand())
        .subcommand(collection::collection_subcommand())
        //
        .get_matches();

    match matches.subcommand() {
        Some(("info", info_matches)) => info::info_matcher(info_matches, &client),
        Some(("collection", collection_matches)) => {
            collection::collection_matcher(collection_matches, &client)
        }
        _ => {
            println!("No subcommand")
        }
    }
}

fn extract_config_from_env() -> Option<(String, Option<String>)> {
    let base_url = std::env::var("DVCLI_URL").ok()?;
    let api_token = std::env::var("DVCLI_TOKEN").ok();
    Some((base_url, api_token))
}
