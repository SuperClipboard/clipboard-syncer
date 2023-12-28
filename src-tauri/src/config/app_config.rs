use std::sync::Arc;

use anyhow::Result;
use log::{error, info};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;

use crate::config::configure::Configure;

#[derive(Debug)]
pub struct AppConfig {
    configure: Arc<RwLock<Configure>>,
}

impl AppConfig {
    pub fn latest() -> Arc<RwLock<Configure>> {
        Self::global().configure.clone()
    }

    pub async fn modify_config(patch: Configure) -> Result<()> {
        // Before merging opt
        match {
            let old_cfg = AppConfig::latest().read().clone();

            if patch.sync_port.is_some() && old_cfg.sync_port.ne(&patch.sync_port) {
                // todo
                info!("Config sync_port changed, need restart the node");
            }

            if patch.graphql_port.is_some() && old_cfg.graphql_port.ne(&patch.graphql_port) {
                // todo
                info!("Config graphql_port changed, need restart the node");
            }

            if patch.store_limit.is_some() && old_cfg.store_limit.ne(&patch.store_limit) {
                // todo
                info!("delete and refresh over-limit store and cache")
            }

            if patch.mdns_find_others.is_some()
                && old_cfg.mdns_find_others.ne(&patch.mdns_find_others)
            {
                // todo
                info!("mdns_find_others changed")
            }

            if patch.auto_paste.is_some() && old_cfg.auto_paste.ne(&patch.auto_paste) {
                // todo
                info!("auto_paste changed")
            }

            <Result<()>>::Ok(())
        } {
            Ok(_) => {
                let inner_cfg = Self::latest();
                let mut inner_cfg = inner_cfg.write();

                // Merge config
                (*inner_cfg).merge(patch);

                // Save to file
                if let Some(e) = inner_cfg.save_to_file().err() {
                    error!("Modify configuration save to file failed: {}", e);
                    return Err(e);
                };
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    // init global configuration
    fn global() -> &'static AppConfig {
        static CONFIG: OnceCell<AppConfig> = OnceCell::new();

        CONFIG.get_or_init(|| AppConfig {
            configure: Arc::new(RwLock::new(Configure::new())),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::config::app_config::AppConfig;
    use crate::config::configure::Configure;

    #[tokio::test]
    async fn test_modify_config() {
        let old_cfg = AppConfig::latest().read().clone();
        println!("Old config: {:?}", old_cfg);

        AppConfig::modify_config(Configure {
            store_limit: Some(101),
            sync_port: None,
            graphql_port: None,
            toggle_window_hotkey: None,
            auto_paste: Some(false),
            mdns_find_others: Some(false),
        })
        .await
        .unwrap();

        println!("New config: {:?}", AppConfig::latest().read());
    }
}
