use std::{env, fs::OpenOptions};

use reqwest::header;
use serde::{Deserialize, Serialize};

fn main() {
    let device_id = env::var("DEVICE_ID").expect("DEVICE_ID must be set");
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    if let Err(e) = record_current_temp(&device_id, &api_key) {
        eprintln!("record_current_temp error: {}", e);
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetMeterPlusStatusResponse {
    pub status_code: u32,
    pub body: GetMeterPlusStatusResponseBody,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetMeterPlusStatusResponseBody {
    pub temperature: f32,
    pub humidity: u8,
}

fn record_current_temp(
    device_id: &str,
    api_key: &str,
) -> Result<(), reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(format!(
            "https://api.switch-bot.com/v1.0/devices/{device_id}/status"
        ))
        .header(header::AUTHORIZATION, api_key)
        .send()?
        .json::<GetMeterPlusStatusResponse>()?;

    let file = OpenOptions::new().write(true).create(true).append(true).open("./data/temp_humidity.csv").unwrap();
    let mut wtr = csv::WriterBuilder::new().has_headers(false).from_writer(file);
    wtr.serialize(resp.body).unwrap();
    Ok(())
}

#[allow(dead_code)]
fn generate_graph_image() {
    unimplemented!()
}
