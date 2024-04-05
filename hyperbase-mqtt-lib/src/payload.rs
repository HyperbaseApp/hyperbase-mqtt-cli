use ahash::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Payload {
    project_id: Uuid,

    token_id: Uuid,
    user: Option<UserPayload>,

    pub collection_id: Uuid,
    data: Option<HashMap<String, Value>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserPayload {
    collection_id: Uuid,
    id: Uuid,
}
