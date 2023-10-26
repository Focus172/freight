use serde::{Deserialize, Serialize};

use super::Service;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Services {
    pub enabled: Vec<Service>,
}

impl Services {
    pub const fn new() -> Self {
        Self {
            enabled: Vec::new(),
        }
    }
}
