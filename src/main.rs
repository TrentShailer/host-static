//! # `host-static`

use core::net::SocketAddr;
use std::{env::current_dir, path::PathBuf};

use argh::FromArgs;
use axum::Router;
use tower_http::services::ServeDir;
use ts_error::ReportProgramExit;

#[derive(Debug, FromArgs)]
/// Host a static site
struct Cli {
    /// the directory that will be the site root
    #[argh(option)]
    pub root: Option<PathBuf>,

    /// whether the site should be exposed publicly to the network
    #[argh(switch)]
    pub public: bool,
}

#[tokio::main]
async fn main() -> ReportProgramExit {
    let Cli { root, public } = argh::from_env();
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
