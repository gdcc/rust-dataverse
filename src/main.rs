use clap::{Parser, Subcommand};
use dvcli::cli::{
    base::SubCommandTrait,
    info::{InfoArgs, InfoCommands},
};

#[derive(Debug, Parser)]
#[command(name = "DVCLI")]
#[command(about = "Control Dataverse from your terminal.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(
        arg_required_else_help = true,
        about = "Get information about the Dataverse instance"
    )]
    Info(InfoArgs),
}

pub fn main() {
    let cli = Cli::parse();
    let (base_url, _) = extract_config_from_env().expect("Missing configuration");
    let client = dvcli::client::BaseClient::new(&base_url, None).unwrap();

    match cli.command {
        Commands::Info(info) => {
            let info_cmd = info.command.unwrap();
            match info_cmd {
                InfoCommands::Version => info_cmd.process(&client),
            }
        }
    }
}

fn extract_config_from_env() -> Option<(String, Option<String>)> {
    let base_url = std::env::var("DVCLI_URL").ok()?;
    let api_token = std::env::var("DVCLI_TOKEN").ok();
    Some((base_url, api_token))
}
