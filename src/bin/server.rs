//! mini-redis server.
//!
//! This file is the entry point for the server implemented in the library. It
//! performs command line parsing and passes the arguments on to
//! `mini_redis::server`.
//!
//! The `clap` crate is used for parsing arguments.

use async_rdma::{DoubleRdmaListener, RdmaListener};
use mini_redis::{server, DEFAULT_PORT1, DEFAULT_PORT2};

use structopt::StructOpt;
use tokio::net::TcpListener;
use tokio::signal;

#[tokio::main]
pub async fn main() -> mini_redis::Result<()> {
    // enable logging
    // see https://docs.rs/tracing for more info
    tracing_subscriber::fmt::try_init()?;

    let cli = Cli::from_args();
    let port1 = cli.port.as_deref().unwrap_or(DEFAULT_PORT1);
    let port2 = cli.port.as_deref().unwrap_or(DEFAULT_PORT2);

    // Bind a TCP listener
    let listener = DoubleRdmaListener::bind(
        &format!("127.0.0.1:{}", port1),
        &format!("127.0.0.1:{}", port2),
    )
    .await?;

    server::run(listener, signal::ctrl_c()).await;

    Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(name = "mini-redis-server", version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = "A Redis server")]
struct Cli {
    #[structopt(name = "port", long = "--port")]
    port: Option<String>,
}
