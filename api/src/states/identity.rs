use std::{sync::Mutex, collections::HashMap};

use uuid::Uuid;

use crate::structs::identity::PrivateIdentity;

pub struct IdentityState {
    // Private UUID -> Identity
    pub private_identities: Mutex<HashMap<Uuid, PrivateIdentity>>,
    // Public UUID -> Identity
    pub public_identities: Mutex<HashMap<Uuid, PrivateIdentity>>
}

impl IdentityState {
    pub fn new() -> Self {
        Self {
            private_identities: Mutex::new(HashMap::new()),
            public_identities: Mutex::new(HashMap::new())
        }
    }
}