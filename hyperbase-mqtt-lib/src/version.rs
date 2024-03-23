pub enum MqttVersion {
    V3,
    V5,
}

impl MqttVersion {
    pub fn to_str(&self) -> &str {
        match self {
            Self::V3 => "v3",
            Self::V5 => "v5",
        }
    }
}
