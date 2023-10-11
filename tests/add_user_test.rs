use std::process::Command;

use assert_cmd::{crate_name, prelude::CommandCargoExt};
use auth_lite::auth;

mod support;

#[tokio::test]
async fn successfully_add_user() -> anyhow::Result<()> {
    let (pool, temp_db) = support::create_test_db().await?;

    let mut command = Command::cargo_bin(crate_name!())?;
    command.args([
        "-d",
        temp_db.path().to_str().unwrap(),
        "add-user",
        "test-user",
    ]);

    let mut session = rexpect::session::spawn_command(command, Some(1000))?;
    session.exp_string("Password: ")?;
    session.send_line("test-password")?;
    session.exp_eof()?;

    let user = sqlx::query!("SELECT password_hash FROM users WHERE username = 'test-user'")
        .fetch_one(&pool)
        .await?;
    assert!(bcrypt::verify("test-password", &user.password_hash)?);

    Ok(())
}

#[tokio::test]
async fn reject_duplicate_users() -> anyhow::Result<()> {
    let (pool, temp_db) = support::create_test_db().await?;
    auth::create_user(&pool, "test-user", "test-password").await?;

    let mut command = Command::cargo_bin(crate_name!())?;
    command.args([
        "-d",
        temp_db.path().to_str().unwrap(),
        "add-user",
        "test-user",
    ]);

    let mut session = rexpect::session::spawn_command(command, Some(1000))?;
    session.exp_string("User test-user already exists")?;

    Ok(())
}
