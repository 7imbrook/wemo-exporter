use hyper::{Request, Body, Uri, Method};

#[derive(Debug, Clone)]
pub struct WemoInsightSwitch {
    target: String,
}

impl WemoInsightSwitch {
    pub fn new(target: String) -> Self {
        Self { target }
    }
}

impl Into<Request<Body>> for WemoInsightSwitch {
    fn into(self) -> Request<Body> {
        let uri = Uri::builder()
            .scheme("http")
            .authority(self.target)
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
            // TODO come back and fix
            .body(Body::from(""))
            .unwrap()
    }
}
