use anyhow::Result;
use log::{error, info};
use sqlx::{Connection, Executor, SqliteConnection, sqlite::SqliteConnectOptions};
use std::path::Path;

pub(crate) async fn init<P: AsRef<Path>>(db_path: P) -> Result<()> {
    let db_path_stringed = db_path.as_ref().to_string_lossy();

    let db_connection_options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true)
        // Enable SQLite foreign keys rules enforcement
       	.pragma("foreign_keys", "ON");

    let mut db_connection = SqliteConnection::connect_with(&db_connection_options)
        .await
        .map_err(|error| {
            error!("Cannot connect to database at {db_path_stringed}");
            error
        })?;
    info!("Connected to database");

    db_connection.execute(r#"
    	CREATE TABLE IF NOT EXISTS users (
     		id INTEGER PRIMARY KEY AUTOINCREMENT,
       		email TEXT NOT NULL UNIQUE COLLATE NOCASE,
         	password TEXT NOT NULL,
          	name TEXT,
           	active BOOLEAN NOT NULL DEFAULT TRUE,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            last_login DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
     	);
    "#).await?;

    info!("Created database schema succesfully");

    Ok(())
}
