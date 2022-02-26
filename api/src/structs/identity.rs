use serde::{Deserialize, Serialize};
use uuid::Uuid;
use ed25519_dalek::PublicKey;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Identity {
    pub name: String,
    pub id: Uuid,
    pub public_key: PublicKey
}

impl Identity {
    pub fn new(name: String, public_key: PublicKey) -> Self {
        let id = Uuid::new_v4();
        Self {
            name,
            id,
            public_key
        }
    }
}

#[derive(Deserialize)]
pub struct IdentityCreateRequest {
    pub name: String,
    pub public_key: PublicKey,
}