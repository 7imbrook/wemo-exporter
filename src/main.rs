mod config;
mod types;
mod insights;

use clap::Parser;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use crate::insights::query_power_draw;
use std::{convert::Infallible, net::SocketAddr};

async fn prometheus() -> Body {
    Body::from("")
}

async fn metrics(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match req.uri().path() {
        "/metrics" => Ok(Response::new(prometheus().await)),
        _ => Ok(Response::new(Body::from(""))),
    }
}

async fn run_server() {
    println!("Running in server mode");
    // Metrics server!
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    let service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(metrics)) });
    let server = Server::bind(&addr).serve(service);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn validate() {
    // match config::load_config() {
    //     Ok(config) => println!("{:?}", config),
    //     Err(e) => println!("Failed: {}", e),
    // };

    query_power_draw().await;

}

// Probably move all above out of here

/// Wemo-exporter a prometheus exporter for wemo insight switches
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Run as server
    #[clap(short, long)]
    server: bool,

    /// Validate config
    #[clap(short, long)]
    validate: bool,
}

#[tokio::main]
async fn main() {
    match Args::parse() {
        Args { server: true, .. } => run_server().await,
        Args { validate: true, .. } => validate().await,
        _ => {}
    }
}
