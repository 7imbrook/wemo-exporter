mod types;
mod config;

use hyper::{
    body::Buf,
    service::{make_service_fn, service_fn},
    Body, Client, Method, Request, Response, Server, Uri,
};
use std::{convert::Infallible, io::Read, net::SocketAddr};
use clap::Parser;

use crate::types::{get_power_body, read_insight_response};

#[derive(Debug)]
enum WemoError {
    FAIL,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Insight {
    state: bool,
    on_since: f32,
    on_for: f32,
    today_on_for: f32,
    instant_power: f32,
}

fn build_request(target: &str) -> Request<Body> {
    let uri = Uri::builder()
        .scheme("http")
        .authority(target)
        .path_and_query("/upnp/control/insight1")
        .build()
        .unwrap();

    return Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(
            "SOAPACTION",
            "\"urn:Belkin:service:insight:1#GetInsightParams\"",
        )
        .header("Content-Type", "text/xml")
        .body(Body::from(get_power_body()))
        .unwrap();
}

async fn query_power_draw() -> Result<Insight, WemoError> {
    let client = Client::builder().build_http();

    let config = config::load_config().unwrap();

    let target = config.targets.first().unwrap();

    let request = build_request(&target);

    let call = client.request(request).await;
    if let Err(_) = call {
        return Err(WemoError::FAIL);
    }
    let res = call.unwrap();

    let status = res.status();

    let body = hyper::body::aggregate(res).await.unwrap();
    let mut reader = body.reader();

    let mut buffer = String::new();
    // Can probably skip this step and just use the reader
    reader.read_to_string(&mut buffer).unwrap();

    if !status.is_success() {
        return Err(WemoError::FAIL);
    }

    let insights = read_insight_response(&buffer);

    // One line parsing is the way to go right?
    let insight = match insights
        .split("|")
        .map(|v| v.parse().unwrap())
        .collect::<Vec<f32>>()
        .as_slice()
    {
        &[power_state, on_since, on_for, today_on_for, _e, _f, _g, instant_power, _i, _j, _k] => {
            Insight {
                state: power_state > 0.0,
                on_for,
                on_since,
                today_on_for,
                instant_power,
            }
        }
        _ => panic!("Oh no"),
    };
    Ok(insight)
}

async fn metrics(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let insight = query_power_draw().await.unwrap();
    let metric = format!("wemo_power_instant_mw {}", insight.instant_power);
    Ok(Response::new(Body::from(metric)))
}

async fn run_server() {
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
    match config::load_config() {
        Ok(config) => println!("{:?}", config),
        Err(e) => println!("Failed: {}", e)
    };
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
        Args {server: true, .. } => run_server().await,
        Args {validate: true, .. } => validate().await,
        _ => {},
    }
}
