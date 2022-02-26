use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::identity::Identity;

#[derive(Serialize, Deserialize, Clone)]
pub struct Channel {
    pub name: String,
    pub id: Uuid,
    pub initiator: Identity,
    pub subscribed: Vec<Identity>,
    pub closed: bool
}

impl Channel {
    pub fn new(name: String, initiator: Identity) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            name,
            initiator,
            subscribed: vec![],
            closed: false,   
        }
    }
}

#[derive(Deserialize)]
pub struct ChannelCreateRequest {
    pub name: String,
    pub identity_id: Uuid
}