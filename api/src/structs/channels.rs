use serde::{Deserialize, Serialize};
use uuid::Uuid;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;

use super::identity::PublicIdentity;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Channel {
    pub name: String,
    pub id: Uuid,
    pub initiator: PublicIdentity,
    pub subscribed: Vec<PublicIdentity>,
    pub closed: bool
}

impl Channel {
    pub fn new(name: String, initiator: PublicIdentity) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            name,
            initiator,
            subscribed: vec![],
            closed: true,   
        }
    }
    pub fn open(&mut self) {
        self.closed = false;
    }
}

#[derive(Deserialize)]
pub struct ChannelCreateRequest {
    pub name: String,
    pub identity_id: Uuid
}

#[derive(Deserialize)]
pub struct ChannelGetRequest {
    pub id: Uuid
}

#[derive(Deserialize)]
pub struct ChannelStartRequest {
    pub id: Uuid,
    pub identity_id: Uuid,
    pub sdp: RTCSessionDescription
}
#[derive(Serialize)]
pub struct ChannelStartResponse {
    pub sdp: RTCSessionDescription
}

#[derive(Deserialize)]
pub struct ChannelJoinRequest {
    pub id: Uuid,
    pub identity_id: Uuid,
    pub sdp: RTCSessionDescription
}
#[derive(Serialize)]
pub struct ChannelJoinResponse {
    pub sdp: RTCSessionDescription
}