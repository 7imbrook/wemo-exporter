use yaserde::ser::to_string;
use yaserde_derive::YaSerialize;

#[derive(Debug, YaSerialize)]
#[yaserde(
    prefix = "s",
    namespace = "s: http://schemas.xmlsoap.org/soap/envelope/",
    namespace = "u: urn:Belkin:service:insight:1"
)]
struct Envelope {
    #[yaserde(rename = "Body", prefix = "s")]
    body: GetInsightParams,
}

#[derive(Debug, YaSerialize)]
struct GetInsightParams {
    #[yaserde(rename = "GetInsightParams", prefix = "u")]
    get_insight_params: InsightParams,
}

#[derive(Debug, YaSerialize)]
struct InsightParams;

pub fn get_power_body() -> String {
    let body = Envelope {
        body: GetInsightParams {
            get_insight_params: InsightParams,
        },
    };
    return to_string(&body).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::types::get_power_body;

    #[test]
    fn test_serialize() {
        println!("\u{001b}[36m{}\u{001b}[0m", get_power_body());
    }
}
