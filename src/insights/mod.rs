#![allow(dead_code)]

mod wemo;

use std::time::Duration;

use futures::future::join_all;
use hyper::{client::HttpConnector, Client};
use hyper_timeout::TimeoutConnector;

use crate::config;

use self::wemo::WemoInsightSwitch;

#[derive(Debug)]
pub enum WemoError {
    FailUnknown,
    CONNECTION,
}

#[derive(Debug)]
pub struct Insight {
    state: bool,
    on_since: f32,
    on_for: f32,
    today_on_for: f32,
    instant_power: f32,
}

trait Connector {
    fn connect() -> Self;
}

impl Connector for Client<TimeoutConnector<HttpConnector>> {
    fn connect() -> Self {
        let connector = HttpConnector::new();
        let mut timeout = TimeoutConnector::new(connector);
        timeout.set_connect_timeout(Some(Duration::from_secs(1)));
        timeout.set_read_timeout(Some(Duration::from_secs(1)));
        Client::builder().build(timeout)
    }
}

async fn process_response(switch: &WemoInsightSwitch) -> Result<String, WemoError> {
    match Client::connect().request(switch.clone().into()).await {
        Ok(res) if res.status().is_success() => Ok(format!("OK {:?}", res)),
        Ok(res) => Ok(format!("Error {:?} {:?}", switch, res)),
        Err(_e) => Err(WemoError::CONNECTION),
    }
}

pub async fn query_power_draw() {
    println!("Starting");
    let config = config::load_config().unwrap();
    let requests = config
        .targets
        .iter()
        .map(|t| WemoInsightSwitch::new(t.to_owned()))
        .collect::<Vec<WemoInsightSwitch>>();

    let responses = join_all(requests.iter().map(process_response)).await;
    println!("{:?}", responses);
}

// async fn next() {
//     let call = client.request(request).await;
//     // if let Err(_) = call {
//     //     return Err(WemoError::FAIL);
//     // }
//     let res = call.unwrap();

//     let status = res.status();

//     let body = hyper::body::aggregate(res).await.unwrap();
//     let mut reader = body.reader();

//     let mut buffer = String::new();
//     // Can probably skip this step and just use the reader
//     reader.read_to_string(&mut buffer).unwrap();

//     let insights = read_insight_response(&buffer);

//     // One line parsing is the way to go right?
//     let insight = match insights
//         .split("|")
//         .map(|v| v.parse().unwrap())
//         .collect::<Vec<f32>>()
//         .as_slice()
//     {
//         &[power_state, on_since, on_for, today_on_for, _e, _f, _g, instant_power, _i, _j, _k] => {
//             Insight {
//                 state: power_state > 0.0,
//                 on_for,
//                 on_since,
//                 today_on_for,
//                 instant_power,
//             }
//         }
//         _ => panic!("Oh no"),
//     };
// }