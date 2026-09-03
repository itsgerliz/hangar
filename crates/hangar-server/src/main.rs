use clap::Parser;
use hangar_core::HangarState;
use std::path::PathBuf;
use tracing_subscriber::{EnvFilter, fmt as tracing_subscriber_builder};

#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
    #[arg(help = "Database to be used")]
    database: PathBuf,
    #[arg(help = "Data to be served")]
    data: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli_args = CliArgs::parse();

    tracing_subscriber_builder()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "INFO".into()))
        .with_level(true)
        .with_target(true)
        .init();

    let hangar_state = HangarState::new(cli_args.database, cli_args.data).await?;

    hangar_state.shutdown();

    Ok(())
}
