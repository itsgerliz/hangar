use anyhow::Result;
use clap::Parser;
use env_logger::Env;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
	#[arg()]
	database: PathBuf
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli_args = CliArgs::parse();

    Ok(())
}
