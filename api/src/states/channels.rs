use std::{sync::Mutex, collections::HashMap};

use uuid::Uuid;

use crate::structs::channels::Channel;

pub struct ChannelState {
    // UUID -> Channel
    pub channels: Mutex<HashMap<Uuid, Channel>>
}

impl ChannelState {
    pub fn new() -> Self {
        Self {
            channels: Mutex::new(HashMap::new())
        }
    }
}