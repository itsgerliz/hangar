use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::path::PathBuf;
use tracing::info;

pub struct HangarState {
    pub database: PathBuf,
    pub data: PathBuf,
    database_pool: SqlitePool,
}

impl HangarState {
    pub async fn new<P1, P2>(database: P1, data: P2) -> Result<Self, sqlx::Error>
    where
        P1: Into<PathBuf>,
        P2: Into<PathBuf>,
    {
        let database = database.into();
        let data = data.into();

        // Foreign keys constraints enforcement is enabled by default by sqlx
        // Enable file creation so new databases are handled automatically
        let database_pool = SqlitePool::connect_with(
            SqliteConnectOptions::new()
                .filename(&database)
                .create_if_missing(true),
        )
        .await?;

        info!("Database in use: {}", database.to_string_lossy());
        info!("Data being served from: {}", data.to_string_lossy());

        Ok(Self {
            database,
            data,
            database_pool,
        })
    }

    pub async fn migrate(&self) -> Result<(), sqlx::Error> {
		sqlx::migrate!("./migrations").run(&self.database_pool).await?;

		Ok(())
	}

    pub async fn shutdown(self) {
   		info!("Server shutdown");

    	self.database_pool.close().await
    }
}
