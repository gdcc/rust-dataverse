use dataverse::cli::base::Matcher;
use dataverse::cli::collection::CollectionSubCommand;
use dataverse::cli::dataset::DatasetSubCommand;
use dataverse::cli::file::FileSubCommand;
use dataverse::cli::info::InfoSubCommand;
use dataverse::client::BaseClient;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "CLI to interact with Dataverse")]
enum DVCLI {
    Info(InfoSubCommand),
    Collection(CollectionSubCommand),
    Dataset(DatasetSubCommand),
    File(FileSubCommand),
}

fn main() {
    let (base_url, api_token) = extract_config_from_env()
        .expect("Please set the DVCLI_URL and DVCLI_TOKEN environment variables.");

    let client = BaseClient::new(&base_url, api_token.as_ref()).expect("Failed to create client.");
    let dvcli = DVCLI::from_args();

    match dvcli {
        DVCLI::Info(command) => command.process(&client),
        DVCLI::Collection(command) => command.process(&client),
        DVCLI::Dataset(command) => command.process(&client),
        DVCLI::File(command) => command.process(&client),
    }
}

fn extract_config_from_env() -> Option<(String, Option<String>)> {
    let base_url = std::env::var("DVCLI_URL").ok()?;
    let api_token = std::env::var("DVCLI_TOKEN").ok();
    Some((base_url, api_token))
}
