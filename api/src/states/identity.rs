use std::{sync::Mutex, collections::HashMap};

use uuid::Uuid;

use crate::structs::identity::Identity;

pub struct IdentityState {
    // UUID -> Identity
    pub identities: Mutex<HashMap<Uuid, Identity>>
}

impl IdentityState {
    pub fn new() -> Self {
        Self {
            identities: Mutex::new(HashMap::new())
        }
    }
}