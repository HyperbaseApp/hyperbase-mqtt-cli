use hyperbase_mqtt_lib::{broker::Broker, payload::Payload};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub broker: Broker,
    pub payload: Payload,
    pub count: Option<u64>,
    pub delay: Option<u64>,
}
