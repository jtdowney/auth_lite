use std::{path::Path, time::Duration};

use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!("db/migrations");

pub async fn open(path: impl AsRef<Path>) -> anyhow::Result<sqlx::SqlitePool> {
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .create_if_missing(true)
        .filename(path)
        .busy_timeout(Duration::from_secs(5))
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

    let pool = sqlx::SqlitePool::connect_with(options).await?;
    MIGRATOR.run(&pool).await?;

    Ok(pool)
}
