mod routes;

use argon2::{Argon2, PasswordHasher, password_hash};
use humantime::format_duration;
use sqlx::{
    migrate,
    sqlite::{SqliteConnectOptions, SqlitePool},
};
use std::{
    io::{self, Write},
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};
use thiserror::Error;
use tokio::{
    net::TcpListener,
    sync::{Notify, OnceCell},
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
    database: PathBuf,
    database_pool: SqlitePool,
    data: PathBuf,
    started_at: OnceCell<Instant>,
    is_ready: Notify,
    do_shutdown: Notify,
}

impl HangarState {
    // Constructor
    pub async fn new<P1, P2>(database: P1, data: P2) -> Result<Arc<Self>, HangarError>
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
    {
        let hangar_state = Self {
            database: database.as_ref().to_path_buf(),
            database_pool: SqlitePool::connect_with(
                SqliteConnectOptions::new()
                    .filename(database.as_ref())
                    // Enable foreign keys constraints enforcement
                    .foreign_keys(true)
                    // Enable file creation so new databases are handled automatically
                    .create_if_missing(true),
            )
            .await?,
            data: data.as_ref().to_path_buf(),
            started_at: OnceCell::new(),
            is_ready: Notify::new(),
            do_shutdown: Notify::new(),
        };

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

    pub fn prepare(self) -> Arc<Self> {
        Arc::new(self)
    }

    pub async fn serve(self: Arc<Self>) -> Result<(), HangarError> {
        let router = routes::router().with_state(Arc::clone(&self));

        let listener = TcpListener::bind("[::]:9070").await?;

        self.ready_tx.send_replace(true);

        info!("Server startup");
        axum::serve(listener, router)
            .with_graceful_shutdown(async move { self.shutdown_handler().await })
            .await?;

        Ok(())
    }

    pub fn is_ready(&self) -> bool {
        let ready_rx = self.ready_tx.subscribe();
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
