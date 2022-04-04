use yaserde::{de::from_str, ser::to_string};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Default, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "s",
    namespace = "s: http://schemas.xmlsoap.org/soap/envelope/",
    namespace = "u: urn:Belkin:service:insight:1"
)]
struct Envelope {
    #[yaserde(rename = "Body", prefix = "s")]
    body: GetInsightParams,
}

#[derive(Debug, YaSerialize, YaDeserialize)]
enum GetInsightParams {
    #[yaserde(prefix = "u")]
    GetInsightParams(InsightParams),
    #[yaserde(prefix = "u")]
    GetInsightParamsResponse(InsightParams),
}

impl Default for GetInsightParams {
    fn default() -> Self {
        GetInsightParams::GetInsightParamsResponse(InsightParams { child: None })
    }
}

#[derive(Default, Debug, YaSerialize, YaDeserialize)]
struct InsightParams {
    #[yaserde(child)]
    child: Option<String>,
}

pub fn get_power_body() -> String {
    let body = Envelope {
        body: GetInsightParams::GetInsightParams(InsightParams { child: None }),
    };
    return to_string(&body).unwrap();
}

pub fn read_insight_response(buffer: &str) {
    println!("\u{001b}[36m{:?}\u{001b}[0m", buffer);
    let parsed: Envelope = from_str(&buffer).unwrap();
    println!("{:?}", parsed);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::types::get_power_body;

    #[test]
    fn test_serialize() {
        assert_eq!(
            fs::read_to_string("tests/data/get_power.xml").unwrap(),
            get_power_body()
        );
    }
}
