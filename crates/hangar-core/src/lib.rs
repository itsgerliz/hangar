mod routes;

use argon2::{Argon2, PasswordHasher, password_hash};
use humantime::format_duration;
use sqlx::{
    migrate,
    sqlite::{SqliteConnectOptions, SqlitePool},
};
use std::{
    io::{self, Write},
    path::PathBuf,
    sync::Arc,
    time::Instant,
};
use thiserror::Error;
use tokio::{
    net::TcpListener,
    sync::watch::{Sender, channel},
};
use tracing::info;

#[derive(Error, Debug)]
pub enum HangarError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    #[error("IO error")]
    IO(#[from] std::io::Error),
    #[error("Password hashing error")]
    PasswordHashing(#[from] password_hash::Error),
}

pub struct HangarState {
    started_at: Instant,
    database: PathBuf,
    data: PathBuf,
    database_pool: SqlitePool,
    shutdown_tx: Sender<bool>,
}

impl HangarState {
    // Constructor
    pub async fn new<P1, P2>(database: P1, data: P2) -> Result<Self, HangarError>
    where
        P1: Into<PathBuf>,
        P2: Into<PathBuf>,
    {
        let started_at = Instant::now();
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

        let (shutdown_tx, _) = channel(false);

        let hangar_state = Self {
            started_at,
            database,
            data,
            database_pool,
            shutdown_tx,
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
        migrate!("./migrations")
            .run(&hangar_state.database_pool)
            .await
            .map_err(|migrate_error| sqlx::Error::from(migrate_error))?;

        if let None = sqlx::query("SELECT 1 FROM hangar LIMIT 1")
            .fetch_optional(&hangar_state.database_pool)
            .await?
        {
            print!("New admin password: ");
            io::stdout().flush()?;

            let mut new_admin_password = String::new();
            io::stdin().read_line(&mut new_admin_password)?;
            let new_admin_password = String::from(new_admin_password.trim());

            let argon2_ctx = Argon2::default();
            let new_admin_password_hash = argon2_ctx
                .hash_password(new_admin_password.as_bytes())?
                .to_string();

            sqlx::query("INSERT INTO hangar (id, admin_password_hash) VALUES (1, $1)")
                .bind(new_admin_password_hash)
                .execute(&hangar_state.database_pool)
                .await?;
        }

        info!("Server ready for startup");

        Ok(hangar_state)
    }

    // Destructor
    pub fn shutdown(&self) {
        // Reason: `send()` only returns an error if there are zero active Receivers on the
        // shutdown channel, this can only happen if `shutdown()` is called before `serve()`
        // is called or after it exits (because `serve()` calls `shutdown_handler()`,
        // which in turn holds the shutdown channel only Receiver)
        // In either case, there is nothing to shut down
        let _ = self.shutdown_tx.send(true);
    }

    pub async fn serve(self: Arc<Self>) -> Result<(), HangarError> {
        let router = routes::router().with_state(Arc::clone(&self));

        let listener = TcpListener::bind("[::]:9070").await?;

        info!("Server startup");
        axum::serve(listener, router)
            .with_graceful_shutdown(async move { self.shutdown_handler().await })
            .await?;

        Ok(())
    }

    async fn shutdown_handler(&self) {
        let mut shutdown_rx = self.shutdown_tx.subscribe();

        // Reason: Error is impossible, the only caller of this function (`serve()`) will
        // always hold at least one instance of Arc<HangarState> (therefore holding HangarState),
        // since the shutdown channel only Sender lives inside HangarState, this error is impossible
        let _ = shutdown_rx.wait_for(|shutdown| *shutdown).await;

        self.database_pool.close().await;

        info!("Server has run for {}", self.uptime());
        info!("Server shutdown");
    }

    fn uptime(&self) -> String {
        Instant::now()
            .checked_duration_since(self.started_at)
            .map(|duration| format_duration(duration).to_string())
            .unwrap_or_default()
    }
}
