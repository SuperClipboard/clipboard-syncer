use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Configure {
    pub record_limit: Option<u32>,
}

impl Configure {
    pub fn new() -> Self {
        Self {
            record_limit: Some(100),
        }
    }
}
