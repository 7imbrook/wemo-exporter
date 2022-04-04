mod types;

use std::io::Read;
use hyper::{body::Buf, Body, Client, Method, Request};

use crate::types::get_power_body;

#[derive(Debug)]
enum WemoError {}

async fn query_power_draw() -> Result<(), WemoError> {
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

    dbg!(res.status());

    let body = hyper::body::aggregate(res).await.unwrap();
    let mut reader = body.reader();

    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    println!("{}", buffer);

    Ok(())
}

#[tokio::main]
async fn main() {
    query_power_draw().await.unwrap();
}
