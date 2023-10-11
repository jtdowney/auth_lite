use assert_fs::NamedTempFile;
use auth_lite::db;

pub async fn create_test_db() -> anyhow::Result<(sqlx::SqlitePool, NamedTempFile)> {
    let temp_db = NamedTempFile::new("auth.db")?;
    let pool = db::open(temp_db.path()).await?;

    Ok((pool, temp_db))
}
