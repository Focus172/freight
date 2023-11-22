use serde::{Deserialize, Serialize};

use super::Service;
use crate::prelude::*;

#[derive(Debug, Default, Serialize, Deserialize, Stub)]
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
