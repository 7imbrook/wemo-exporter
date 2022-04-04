use yaserde::{de::from_str, ser::to_string_with_config};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Default, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "s",
    namespace = "s: http://schemas.xmlsoap.org/soap/envelope/"
    namespace = "u: urn:Belkin:service:insight:1"
)]
struct Envelope {
    #[yaserde(rename = "Body", prefix = "s", default_namespace = "s")]
    body: BodyParams,
}

#[derive(Debug, YaSerialize, YaDeserialize)]
enum BodyParams {
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
struct InsightParams {
    #[yaserde(child)]
    child: Option<String>,
}

mod response {
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

pub fn get_power_body() -> String {
    let yaserde_cfg = yaserde::ser::Config {
        perform_indent: true,
        ..Default::default()
    };
    let body = Envelope {
        body: BodyParams::GetInsightParams(InsightParams { child: None }),
    };
    return to_string_with_config(&body, &yaserde_cfg).unwrap();
}

pub fn read_insight_response(buffer: &str) -> String {
    let parsed: response::Envelope = from_str(&buffer).unwrap();
    String::from(parsed.body.response.insight)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::types::{get_power_body, read_insight_response};

    #[test]
    fn test_serialize() {
        assert_eq!(
            fs::read_to_string("tests/data/get_power.xml").unwrap(),
            get_power_body()
        );
    }
    
    #[test]
    fn test_deserialize() {
        let buffer = fs::read_to_string("tests/data/get_power_response.xml").unwrap();
        assert_eq!(
            read_insight_response(&buffer),
            "8|1648943256|0|0|63636|99560|294|0|0|51821989.000000|8000"
        );
    }
}
