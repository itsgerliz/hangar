use anyhow::Result;
use log::{error, info};
use sqlx::{
    Connection, Executor, query,
    sqlite::{SqliteConnectOptions, SqliteConnection},
};
use std::path::Path;

use crate::sql;

pub(crate) async fn init_db<P: AsRef<Path>>(db_path: P) -> Result<()> {
    let db_connection_options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true)
        // Enable SQLite foreign keys rules enforcement
        .pragma("foreign_keys", "ON");

    let mut db_connection = SqliteConnection::connect_with(&db_connection_options)
        .await
        .map_err(|error| {
            error!("Cannot connect to database");
            error
        })?;
    info!("Connected to database");

    db_connection
        .execute(sql::CREATE_SCHEMA)
        .await
        .map_err(|error| {
            error!("Could not create database schema");
            error
        })?;
    query(sql::INSERT_STASHDESK_VERSION)
        .bind(env!("CARGO_PKG_VERSION"))
        .execute(&mut db_connection)
        .await
        .map_err(|error| {
            error!("Could not insert metadata into database");
            error
        })?;
    info!("Created database schema succesfully");

    Ok(())
}

pub(crate) async fn setup_pool() -> Result<()> {
    Ok(())
}

pub(crate) async fn start() -> Result<()> {
    Ok(())
}
