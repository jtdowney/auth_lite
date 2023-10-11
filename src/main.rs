use std::{
    env,
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};

use auth_lite::{auth, db, server};
use clap::{Parser, Subcommand};

/// A very simple auth server
#[derive(Debug, Parser)] // requires `derive` feature
struct Cli {
    #[arg(short, long, env = "DATABASE_PATH", default_value = "auth.db")]
    database: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Add a new user (alias add)
    #[command(alias = "add", arg_required_else_help = true)]
    AddUser {
        /// The username of the user to add
        username: String,
    },
    /// Remove a user (alias rm, remove)
    #[command(alias = "rm", alias = "remove", arg_required_else_help = true)]
    RemoveUser {
        /// The username of the user to remove
        username: String,
    },
    /// Change a user's password (alias passwd)
    #[command(alias = "passwd", arg_required_else_help = true)]
    ChangePassword {
        /// The username of the user to change the password for
        username: String,
    },
    /// List all users (alias ls, list)
    #[command(alias = "ls", alias = "list")]
    ListUsers,
    /// Start the server
    Serve {
        /// The port to listen on
        #[arg(short, long, env = "PORT", default_value = "4000")]
        port: u16,
        /// The address to bind to
        #[arg(short, long, env = "BIND", default_value = "0.0.0.0")]
        bind: IpAddr,
    },
}

fn setup_tracing() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_tracing();

    let cli = Cli::parse();
    let pool = db::open(&cli.database).await?;

    match cli.command {
        Command::AddUser { username } => {
            let count = sqlx::query!(
                "SELECT count(1) AS count FROM users WHERE username = ?",
                username
            )
            .fetch_one(&pool)
            .await?
            .count;

            if count > 0 {
                eprintln!("User {username} already exists");
                return Ok(());
            }

            let password = rpassword::prompt_password("Password: ")?;
            auth::create_user(&pool, &username, &password).await?;
        }
        Command::RemoveUser { username } => {
            let count = sqlx::query!(
                "SELECT count(1) AS count FROM users WHERE username = ?",
                username
            )
            .fetch_one(&pool)
            .await?
            .count;

            if count == 0 {
                eprintln!("User {username} doesn't exist");
                return Ok(());
            }

            auth::remove_user(&pool, &username).await?;
        }
        Command::ChangePassword { username } => {
            let count = sqlx::query!(
                "SELECT count(1) AS count FROM users WHERE username = ?",
                username
            )
            .fetch_one(&pool)
            .await?
            .count;

            if count == 0 {
                eprintln!("User {username} doesn't exist");
                return Ok(());
            }

            let password = rpassword::prompt_password("Password: ")?;
            auth::change_password(&pool, &username, &password).await?;
        }
        Command::ListUsers => {
            let users = sqlx::query!("SELECT username FROM users ORDER BY username")
                .fetch_all(&pool)
                .await?;

            for user in users {
                println!("{}", user.username);
            }
        }
        Command::Serve { port, bind } => {
            let addr = SocketAddr::from((bind, port));
            server::start(addr, pool).await?;
        }
    }

    Ok(())
}
