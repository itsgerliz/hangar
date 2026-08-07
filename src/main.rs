mod bootstrap;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use env_logger::Env;
use log::info;

#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
    #[command(subcommand)]
    command: CliArgsSubcommands,
}

#[derive(Subcommand)]
enum CliArgsSubcommands {
    /// Initialize a new database
    Init {
        /// Path to initialize the database at
        database_path: String,
    },
    /// Proceed with an existing database
    Start {
        /// Path of the database to be used
        database_path: String,
        /// Path of the data directory to be used
        datadir_path: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli_args = CliArgs::parse();

    match cli_args.command {
        CliArgsSubcommands::Init { database_path } => {
            info!("Will attempt to initialize database at {database_path}");
            bootstrap::init(&database_path).await?;
            info!("Successfully intialized database at {database_path}");
            Ok(())
        }
        CliArgsSubcommands::Start {
            database_path,
            datadir_path,
        } => {
            info!("Using database at {}", database_path);
            info!("Using data directory at {}", datadir_path);

            Ok(())
        }
    }
}
