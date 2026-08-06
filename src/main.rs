mod init;

use anyhow::{Ok, Result};
use clap::Parser;
use env_logger::Env;
use log::info;

#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
    #[arg(
        short,
        long,
        conflicts_with = "data",
        conflicts_with = "database",
        help = "Initialize a new database on path"
    )]
    init: Option<String>,
    #[arg(
        short = 'd',
        long,
        conflicts_with = "init",
        required_unless_present = "init",
        help = "Data directory path"
    )]
    data: Option<String>,
    #[arg(
        short = 'D',
        long,
        conflicts_with = "init",
        required_unless_present = "init",
        help = "Database path"
    )]
    database: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli_args = CliArgs::parse();

    if let Some(db_path) = cli_args.init {
		info!("Will attempt to initialize database at {db_path}");
        init::init(&db_path).await?;
        info!("Successfully intialized database at {db_path}");
    }

    Ok(())
}
