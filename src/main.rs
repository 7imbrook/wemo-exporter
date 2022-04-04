use std::io::Read;

use hyper::{body::{HttpBody, Buf}, Client, Method, Request, Body};

#[derive(Debug)]
enum WemoError {
}

const INSIGHT_BODY: &str = "<s:Envelope xmlns:s=\"http://schemas.xmlsoap.org/soap/envelope/\" s:encodingStyle=\"http://schemas.xmlsoap.org/soap/encoding/\">
    <s:Body>
        <u:GetInsightParams xmlns:u=\"urn:Belkin:service:insight:1\"></u:GetInsightParams>
    </s:Body>
</s:Envelope>";

async fn query_power_draw() -> Result<(), WemoError> {
    let request_power = Request::builder()
        .method(Method::POST)
        .uri("http://10.1.229.62:49153/upnp/control/insight1")
        .header("SOAPACTION", "\"urn:Belkin:service:insight:1#GetInsightParams\"")
        .header("Content-Type", "text/xml")
        .body(Body::from(INSIGHT_BODY))
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
