use serde::Deserialize;

#[derive(Deserialize)]
pub struct Broker {
    pub host: String,
    pub port: u16,
    pub topic: String,
    pub token_id: String,
    pub token: String,
    pub mqtt_version: Option<String>,
}
