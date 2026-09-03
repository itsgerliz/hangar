mod routes;

use humantime::format_duration;
use sqlx::{
    migrate,
    sqlite::{SqliteConnectOptions, SqlitePool},
};
use tokio::net::TcpListener;
use std::{path::PathBuf, time::Instant};
use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
pub enum HangarError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    #[error("IO error")]
    IO(#[from] std::io::Error)
}

pub struct HangarState {
	started_at: Instant,
    database: PathBuf,
    database_pool: SqlitePool,
    data: PathBuf,
}

impl HangarState {
    pub async fn new<P1, P2>(database: P1, data: P2) -> Result<Self, HangarError>
    where
        P1: Into<PathBuf>,
        P2: Into<PathBuf>,
    {
        let started_at = Instant::now();

        let database = database.into();

        // Foreign keys constraints enforcement is enabled by default by sqlx
        // Enable file creation so new databases are handled automatically
        let database_pool = SqlitePool::connect_with(
            SqliteConnectOptions::new()
                .filename(&database)
                .create_if_missing(true),
        )
        .await?;

        let data = data.into();

        let hangar_state = Self {
        	started_at,
            database,
            database_pool,
            data,
        };

        info!(
            "Database in use: {}",
            hangar_state.database.to_string_lossy()
        );
        info!(
            "Data being served from: {}",
            hangar_state.data.to_string_lossy()
        );

        info!("Running database migrations");
        hangar_state.migrate().await?;

        info!("Server starting up");
        hangar_state.serve().await?;

        Ok(hangar_state)
    }

    pub fn shutdown(self) {
    	let uptime = self.uptime();

        info!("Server has run {}", uptime);
        info!("Server shutdown");
    }

    // Triggered by constructor
    async fn migrate(&self) -> Result<(), HangarError> {
	   	Ok(
		    migrate!("./migrations")
		        .run(&self.database_pool)
		        .await
		        .map_err(|migrate_error| sqlx::Error::from(migrate_error))?
	    )
    }

    // Triggered by constructor
    async fn serve(&self) -> Result<(), HangarError> {
   		let router = routes::router();

     	let listener = TcpListener::bind("[::]:9070").await?;

     	info!("Server startup");
      	axum::serve(listener, router).await?;

    	Ok(())
    }

    fn uptime(&self) -> String {
	   	Instant::now()
	        .checked_duration_since(self.started_at)
	        .map(|duration| format_duration(duration).to_string())
	       	.unwrap_or_default()
    }
}
