use std::env;

use reqwest::header;
use serde::Deserialize;

fn main() {
    let device_id = env::var("DEVICE_ID").expect("DEVICE_ID must be set");
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let t = env::var("T").expect("T must be set");
    let sign = env::var("SIGN").expect("SIGN must be set");
    if let Err(e) = check_current_temp(&device_id, &api_key, &t, &sign) {
        eprintln!("check_current_temp error: {}", e);
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetMeterPlusStatusResponse {
    pub status_code: u32,
    pub body: GetMeterPlusStatusResponseBody,
    pub message: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetMeterPlusStatusResponseBody {
    pub temperature: f32,
    pub humidity: u8,
}

fn check_current_temp(
    device_id: &str,
    api_key: &str,
    t: &str,
    sign: &str,
) -> Result<(), reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(api_key).unwrap(),
    );
    headers.insert("t", header::HeaderValue::from_str(t).unwrap());
    headers.insert(
        "sign",
        header::HeaderValue::from_str(sign).unwrap(),
    );
    headers.insert("nonce", header::HeaderValue::from_static(""));
    let resp = client
        .get(format!(
            "https://api.switch-bot.com/v1.1/devices/{device_id}/status"
        ))
        .headers(headers)
        .send()?
        .json::<GetMeterPlusStatusResponse>()?;
    println!("Current temperature: {:.1}", resp.body.temperature);

    Ok(())
}

#[allow(dead_code)]
fn generate_graph_image() {
    unimplemented!()
}
