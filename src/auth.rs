#[cfg(not(debug_assertions))]
const BCRYPT_COST: u32 = bcrypt::DEFAULT_COST;
#[cfg(debug_assertions)]
const BCRYPT_COST: u32 = 4;

pub async fn authenticate(
    pool: &sqlx::SqlitePool,
    username: &str,
    password: &str,
) -> anyhow::Result<bool> {
    let row = sqlx::query!(
        "SELECT password_hash FROM users WHERE username = ?",
        username
    )
    .fetch_optional(pool)
    .await?;

    if let Some(user) = row {
        let verified = bcrypt::verify(password, &user.password_hash)?;
        Ok(verified)
    } else {
        Ok(false)
    }
}

pub async fn create_user(
    pool: &sqlx::SqlitePool,
    username: &str,
    password: &str,
) -> anyhow::Result<()> {
    let password_hash = bcrypt::hash(password, BCRYPT_COST)?;
    sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES (?, ?)",
        username,
        password_hash
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn change_password(
    pool: &sqlx::SqlitePool,
    username: &str,
    password: &str,
) -> anyhow::Result<()> {
    let password_hash = bcrypt::hash(password, BCRYPT_COST)?;
    sqlx::query!(
        "UPDATE users SET password_hash = ? WHERE username = ?",
        password_hash,
        username,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_user(pool: &sqlx::SqlitePool, username: &str) -> anyhow::Result<()> {
    sqlx::query!("DELETE FROM users WHERE username = ?", username)
        .execute(pool)
        .await?;

    Ok(())
}
