use std::sync::Arc;

use anyhow::Result;
use log::{error, info};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;

use crate::config::configure::Configure;
use crate::sync::server::ServerHandler;

#[derive(Debug)]
pub struct AppConfig {
    configure: Arc<RwLock<Configure>>,
}

const SHUTDOWN_WAIT_MILLIS: u64 = 3000;

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
                info!("Config sync_port changed, restarting the rpc server");
                ServerHandler::global().lock().await.shutdown().await?;
                tokio::time::sleep(tokio::time::Duration::from_millis(SHUTDOWN_WAIT_MILLIS)).await;
                ServerHandler::global()
                    .lock()
                    .await
                    .start(patch.sync_port.as_ref().unwrap())
                    .await?;
            }

            if patch.record_limit_threshold.is_some()
                && old_cfg
                    .record_limit_threshold
                    .ne(&patch.record_limit_threshold)
            {
                info!("config record_limit_threshold changed!");
            }

            if patch.store_limit.is_some() && old_cfg.store_limit.ne(&patch.store_limit) {
                // todo
                info!("delete and refresh over-limit store and cache")
            }

            if patch.sync_server_addr_list.is_some()
                && old_cfg
                    .sync_server_addr_list
                    .ne(&patch.sync_server_addr_list)
            {
                // todo
                info!("re-register add servers")
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
    use crate::sync::server::ServerHandler;
    use std::collections::HashSet;

    #[tokio::test]
    async fn test_modify_config() {
        let old_cfg = AppConfig::latest().read().clone();
        println!("Old config: {:?}", old_cfg);

        AppConfig::modify_config(Configure {
            store_limit: Some(101),
            sync_port: None,
            record_limit_threshold: Some(51),
            sync_server_addr_list: Some(HashSet::from(["127.0.0.1".to_string()])),
        })
        .await
        .unwrap();

        println!("New config: {:?}", AppConfig::latest().read());
    }

    #[tokio::test]
    async fn test_restart() {
        // Start
        tokio::spawn(async {
            let sync_port;
            {
                sync_port = AppConfig::latest().read().sync_port.clone().unwrap();
            }
            ServerHandler::global()
                .lock()
                .await
                .start(&sync_port)
                .await
                .unwrap();
        });

        // Change port to trigger a restart
        tokio::spawn(async {
            AppConfig::modify_config(Configure {
                sync_port: Some("12222".to_string()),
                store_limit: None,
                record_limit_threshold: None,
                sync_server_addr_list: None,
            })
            .await
            .unwrap();
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;

        tokio::spawn(async {
            ServerHandler::global()
                .lock()
                .await
                .shutdown()
                .await
                .unwrap();
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    }
}
