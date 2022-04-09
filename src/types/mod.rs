#![allow(dead_code)]

use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Default, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "s",
    namespace = "s: http://schemas.xmlsoap.org/soap/envelope/"
    namespace = "u: urn:Belkin:service:insight:1"
)]
pub struct Envelope {
    #[yaserde(rename = "Body", prefix = "s", default_namespace = "s")]
    pub body: BodyParams,
}

#[derive(Debug, YaSerialize, YaDeserialize)]
pub enum BodyParams {
    #[yaserde(prefix = "u")]
    GetInsightParams(InsightParams),
    #[yaserde(prefix = "u")]
    GetInsightParamsResponse(InsightParams),
}

impl Default for BodyParams {
    fn default() -> Self {
        BodyParams::GetInsightParamsResponse(InsightParams { child: None })
    }
}

#[derive(Default, Debug, YaSerialize, YaDeserialize)]
pub struct InsightParams {
    #[yaserde(child)]
    pub child: Option<String>,
}

pub mod response {
    use yaserde_derive::{YaDeserialize, YaSerialize};

    #[derive(Default, Debug, YaDeserialize, YaSerialize)]
    #[yaserde(
        prefix = "s",
        namespace = "s: http://schemas.xmlsoap.org/soap/envelope/",
        namespace = "urn:Belkin:service:metainfo:1"
    )]
    pub struct Envelope {
        #[yaserde(prefix = "s", rename = "Body")]
        pub body: Body,
    }

    #[derive(Default, Debug, YaDeserialize, YaSerialize)]
    pub struct Body {
        #[yaserde(prefix = "u", rename = "GetInsightParamsResponse")]
        pub response: GetInsightParamsResponse,
    }

    #[derive(Default, Debug, YaDeserialize, YaSerialize)]
    pub struct GetInsightParamsResponse {
        #[yaserde(rename = "InsightParams")]
        pub insight: String,
    }
}