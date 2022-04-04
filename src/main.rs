mod types;

use hyper::{body::Buf, Body, Client, Method, Request, service::{make_service_fn, service_fn}, Server, Response};
use tokio::time::sleep;
use std::{io::Read, time::Duration, net::SocketAddr, convert::Infallible};

use crate::types::{get_power_body, read_insight_response};

#[derive(Debug)]
enum WemoError {}

#[derive(Debug)]
struct Insight {
    state: bool,
    on_since: f32,
    on_for: f32,
    today_on_for: f32,
    instant_power: f32,
}

async fn query_power_draw() -> Result<Insight, WemoError> {
    let request_power = Request::builder()
        .method(Method::POST)
        .uri("http://10.1.229.62:49153/upnp/control/insight1")
        .header(
            "SOAPACTION",
            "\"urn:Belkin:service:insight:1#GetInsightParams\"",
        )
        .header("Content-Type", "text/xml")
        .body(Body::from(get_power_body()))
        .expect("Build");

    let client = Client::new();
    let res = client.request(request_power).await.unwrap();
    let status = res.status();

    let body = hyper::body::aggregate(res).await.unwrap();
    let mut reader = body.reader();

    let mut buffer = String::new();
    // Can probably skip this step and just use the reader
    reader.read_to_string(&mut buffer).unwrap();

    if !status.is_success() {
        panic!("{}", buffer);
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

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0,0,0,0], 3001));

    let service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(metrics))
    });

    let server = Server::bind(&addr).serve(service);
    
    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
