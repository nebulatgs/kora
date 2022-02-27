use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PrivateIdentity {
    pub name: String,
    pub public_id: Uuid,
    pub private_id: Uuid,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PublicIdentity {
    pub name: String,
    pub public_id: Uuid,
}

impl From<PrivateIdentity> for PublicIdentity {
    fn from(identity: PrivateIdentity) -> Self {
        Self {
            name: identity.name,
            public_id: identity.public_id
        }
    }
}

impl PrivateIdentity {
    pub fn new(name: String) -> Self {
        let (public_id, private_id) = (Uuid::new_v4(), Uuid::new_v4());
        Self {
            name,
            public_id,
            private_id
        }
    }
}

#[derive(Deserialize)]
pub struct IdentityCreateRequest {
    pub name: String,
}