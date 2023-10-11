use std::process::Command;

use assert_cmd::{crate_name, prelude::CommandCargoExt};
use auth_lite::auth;

mod support;

#[tokio::test]
async fn successfully_remove_user() -> anyhow::Result<()> {
    let (pool, temp_db) = support::create_test_db().await?;
    auth::create_user(&pool, "test-user", "test-password").await?;

    let mut command = Command::cargo_bin(crate_name!())?;
    command.args([
        "-d",
        temp_db.path().to_str().unwrap(),
        "remove-user",
        "test-user",
    ]);

    let mut session = rexpect::session::spawn_command(command, Some(1000))?;
    session.exp_eof()?;

    let row = sqlx::query!("SELECT count(1) as count FROM users WHERE username = 'test-user'")
        .fetch_one(&pool)
        .await?;
    assert_eq!(row.count, 0);

    Ok(())
}

#[tokio::test]
async fn reject_missing_users() -> anyhow::Result<()> {
    let (_, temp_db) = support::create_test_db().await?;

    let mut command = Command::cargo_bin(crate_name!())?;
    command.args([
        "-d",
        temp_db.path().to_str().unwrap(),
        "remove-user",
        "test-user",
    ]);

    let mut session = rexpect::session::spawn_command(command, Some(1000))?;
    session.exp_string("User test-user doesn't exist")?;

    Ok(())
}
