mod bootstrap;
mod sql;

use anyhow::Result;
use clap::{Parser, Subcommand};
use env_logger::Env;
use log::info;
use std::path::PathBuf;

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
        /// Path of the data directory to be used
        datadir_path: String,
        /// Path of the database to be used
        database_path: String,
    },
}

struct DataState {
    path: PathBuf,
}

struct DbState {
    path: PathBuf,
}

struct AppState {
	data_state: DataState,
    db_state: DbState,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli_args = CliArgs::parse();

    match cli_args.command {
        CliArgsSubcommands::Init { database_path } => {
            info!("Will attempt to initialize database at {database_path}");
            bootstrap::init_db(&database_path).await?;
            info!("Successfully intialized database!");

            Ok(())
        }
        CliArgsSubcommands::Start {
        	datadir_path,
            database_path,
        } => {
            let app_state = AppState {
	            data_state: DataState {
	                path: PathBuf::from(datadir_path),
	            },
                db_state: DbState {
                    path: PathBuf::from(database_path),
                },
            };

            info!("Using data directory at {}", app_state.data_state.path.display());
            info!("Using database at {}", app_state.db_state.path.display());

            bootstrap::start().await?;
            info!("Server up and running, listening for requests...");

            Ok(())
        }
    }
}
