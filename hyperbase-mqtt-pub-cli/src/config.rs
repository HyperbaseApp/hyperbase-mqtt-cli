use hyperbase_mqtt_lib::payload::Payload;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub topic: String,
    pub payload: Payload,
    pub count: Option<u64>,
    pub delay: Option<u64>,
    pub mqtt_version: Option<String>,
}
