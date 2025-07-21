//! # `host-static`

use core::net::SocketAddr;
use std::{env::current_dir, path::PathBuf};

use axum::Router;
use clap::Parser;
use tower_http::services::ServeDir;
use ts_rust_helper::error::ReportProgramExit;

#[derive(Debug, Parser)]
/// Host a static site
struct Cli {
    /// The directory that will be the site root.
    pub root: Option<PathBuf>,

    /// If the site should be exposed publicly to the network.
    #[arg(long)]
    pub public: bool,
}

#[tokio::main]
async fn main() -> ReportProgramExit {
    let Cli { root, public } = Cli::parse();
    let root = root.unwrap_or_else(|| current_dir().unwrap());
    let serve_dir = ServeDir::new(root);
    let router = Router::new().fallback_service(serve_dir);

    let addr = if public {
        SocketAddr::from(([0, 0, 0, 0], 0))
    } else {
        SocketAddr::from(([127, 0, 0, 1], 0))
    };
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let addr = listener.local_addr()?;
    println!("Serving to http://localhost:{} ({})", addr.port(), addr);
    axum::serve(listener, router).await?;

    Ok(())
}
