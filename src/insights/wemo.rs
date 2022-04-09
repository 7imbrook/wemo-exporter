use crate::types::{Envelope, BodyParams, InsightParams};

use hyper::{Request, Body, Uri, Method};
use yaserde::ser::to_string_with_config;

#[derive(Debug, Clone)]
pub struct WemoInsightSwitch {
    pub target: String,
}

impl WemoInsightSwitch {
    pub fn new(target: String) -> Self {
        Self { target }
    }
}

impl WemoInsightSwitch {
    fn body(&self) -> String {
        let yaserde_cfg = yaserde::ser::Config {
            perform_indent: true,
            ..Default::default()
        };
        let body = Envelope {
            body: BodyParams::GetInsightParams(InsightParams { child: None }),
        };
        return to_string_with_config(&body, &yaserde_cfg).unwrap();
    }
}

impl Into<Request<Body>> for WemoInsightSwitch {
    fn into(self) -> Request<Body> {
        let uri = Uri::builder()
            .scheme("http")
            .authority(self.target.to_owned())
            .path_and_query("/upnp/control/insight1")
            .build()
            .unwrap();

        Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header(
                "SOAPACTION",
                "\"urn:Belkin:service:insight:1#GetInsightParams\"",
            )
            .header("Content-Type", "text/xml")
            .body(Body::from(self.body()))
            .unwrap()
    }
}
