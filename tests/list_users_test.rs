use std::process::Command;

use assert_cmd::{crate_name, prelude::CommandCargoExt};
use auth_lite::auth;

mod support;

#[tokio::test]
async fn list_users() -> anyhow::Result<()> {
    let (pool, temp_db) = support::create_test_db().await?;

    auth::create_user(&pool, "b-test-user", "test-password").await?;
    auth::create_user(&pool, "a-test-user", "test-password").await?;

    let mut command = Command::cargo_bin(crate_name!())?;
    command.args(["-d", temp_db.path().to_str().unwrap(), "list-users"]);

    let mut session = rexpect::session::spawn_command(command, Some(1000))?;
    session.exp_string("a-test-user")?;
    session.exp_string("b-test-user")?;

    Ok(())
}
