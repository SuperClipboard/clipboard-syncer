use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Configure {
    pub store_limit: u32,
    pub sync_port: String,
    pub record_limit_threshold: usize,
    pub sync_server_addr_list: Vec<String>,
}

impl Default for Configure {
    fn default() -> Self {
        Self {
            store_limit: 100,
            sync_port: "18888".to_string(),
            record_limit_threshold: 50,
            sync_server_addr_list: vec![],
        }
    }
}

impl Configure {
    pub fn new() -> Self {
        Default::default()
    }
}
