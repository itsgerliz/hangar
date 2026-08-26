use anyhow::Result;
use clap::Parser;
use env_logger::Env;
use hangar_core::HangarState;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
    #[arg(help = "Database to be used")]
    database: PathBuf,
    #[arg(help = "Data directory to be served")]
    data_directory: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli_args = CliArgs::parse();

    let hangar_state = HangarState::new(cli_args.database, cli_args.data_directory);

    Ok(())
}
