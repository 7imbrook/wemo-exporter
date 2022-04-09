mod config;
mod insights;
mod types;

use crate::insights::query_power_draw;
use clap::Parser;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use insights::Insight;
use std::{convert::Infallible, fmt::Display, net::SocketAddr};

#[derive(Debug)]
struct Metric<'a> {
    name: &'a str,
    value: String,

    switch: &'a Insight,
}

impl Display for Metric<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let labels = format!("{{test=\"local\", target=\"{}\"}}", self.switch.target);

        write!(f, "{}{} {}", self.name, labels, self.value)
    }
}

impl Insight {
    fn metrics(&self) -> Vec<Metric> {
        vec![
            Metric {
                name: "wemo_instant_power",
                value: self.instant_power.to_string(),
                switch: self,
            },
            Metric {
                name: "wemo_power_state",
                value: format!("{}", self.state as u8),
                switch: self,
            },
        ]
    }
}

async fn prometheus() -> Body {
    let inights = query_power_draw()
        .await
        .iter()
        .map(|i| i.metrics())
        .flatten()
        .fold(String::new(), |a, b| format!("{}\n{}", a, b));

    Body::from(inights)
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
