use std::error::Error;

use colored::Colorize;
use structopt::StructOpt;

use dataverse::cli::base::Matcher;
use dataverse::cli::collection::CollectionSubCommand;
use dataverse::cli::dataset::DatasetSubCommand;
use dataverse::cli::file::FileSubCommand;
use dataverse::cli::info::InfoSubCommand;
use dataverse::client::BaseClient;

static HEADER: &str = r#"
--- Dataverse Command Line Interface (DVCLI) ---
"#;

// This is the basic overall structure of the CLI
// Subcommands are defined in their respective modules
// and are processed here.
#[derive(StructOpt, Debug)]
#[structopt(about = "CLI to interact with Dataverse")]
enum DVCLI {
    Info(InfoSubCommand),
    Collection(CollectionSubCommand),
    Dataset(DatasetSubCommand),
    File(FileSubCommand),
}

fn main() {
    let client = setup_client().expect("Failed to set up client.");
    let dvcli = DVCLI::from_args();

    if atty::is(atty::Stream::Stdout) {
        println!("{}", HEADER.bold());
    }

    match dvcli {
        DVCLI::Info(command) => command.process(&client),
        DVCLI::Collection(command) => command.process(&client),
        DVCLI::Dataset(command) => command.process(&client),
        DVCLI::File(command) => command.process(&client),
    }
}

fn setup_client() -> Result<BaseClient, Box<dyn Error>> {
    let (base_url, api_token) = extract_config_from_env();
    let client = BaseClient::new(&base_url, api_token.as_ref())?;
    Ok(client)
}

// This function extracts the base URL and API token from the environment
// variables DVCLI_URL and DVCLI_TOKEN, respectively.
fn extract_config_from_env() -> (String, Option<String>) {
    let base_url = std::env::var("DVCLI_URL").ok();
    let api_token = std::env::var("DVCLI_TOKEN").ok();

    // If there is no base URL, return None
    if base_url.is_none() {
        panic!("No base URL provided. Please set the DVCLI_URL environment variable.");
    }

    (base_url.unwrap(), api_token)
}