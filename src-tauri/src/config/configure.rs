use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::path::Path;

use anyhow::Result;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};

use crate::utils::dir::config_path;
use crate::utils::json;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Configure {
    pub store_limit: Option<u32>,
    pub sync_port: Option<String>,
    pub record_limit_threshold: Option<usize>,
    pub sync_server_addr_list: Option<HashSet<String>>,
}

impl Default for Configure {
    fn default() -> Self {
        Self {
            store_limit: Some(100),
            sync_port: Some("18888".to_string()),
            record_limit_threshold: Some(50),
            sync_server_addr_list: Some(HashSet::new()),
        }
    }
}

impl Configure {
    pub fn new() -> Self {
        let config_path = config_path().unwrap();
        fs::create_dir_all(config_path.parent().unwrap()).unwrap();

        // No configuration file yet
        if !Path::new(&config_path).exists() {
            info!("Config file initialized at {:?}", config_path);
            File::create(&config_path).unwrap();

            let default_cfg = Configure::default();
            if let Some(e) = json::save(&config_path, &default_cfg).err() {
                error!("Save default configuration file failed: {}", e)
            }
            return default_cfg;
        }

        // Has configuration file, load and merge
        let loaded_cfg = match json::read::<Configure>(&config_path) {
            Ok(c) => c,
            Err(e) => {
                error!("Load configuration file failed: {}", e);
                Configure::default()
            }
        };

        let mut merged_config = Configure::default();
        merged_config.merge(loaded_cfg);

        info!("Config file loaded success!");
        debug!("Current configuration: {:#?}", merged_config);

        merged_config
    }

    pub fn save_to_file(&self) -> Result<()> {
        json::save(&config_path()?, &self)
    }

    // Use merge to avoid adding new features!
    pub fn merge(&mut self, other: Self) {
        macro_rules! merge {
            ($key: tt) => {
                if other.$key.is_some() {
                    self.$key = other.$key;
                }
            };
        }
        merge!(store_limit);
        merge!(sync_port);
        merge!(record_limit_threshold);
        merge!(sync_server_addr_list);
    }
}

#[cfg(test)]
mod tests {
    use crate::config::configure::Configure;
    use crate::utils::dir::config_path;
    use std::path::Path;

    #[test]
    fn test_new_config() {
        let cfg = Configure::new();
        println!("{:?}", cfg);
        assert!(Path::new(&config_path().unwrap()).exists())
    }
}
