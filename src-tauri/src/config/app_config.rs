use std::sync::Arc;

use once_cell::sync::OnceCell;
use parking_lot::Mutex;

use crate::config::configure::Configure;

#[derive(Debug)]
pub struct AppConfig {
    configure: Arc<Mutex<Configure>>,
}

impl AppConfig {
    // init global configuration
    pub fn global() -> &'static AppConfig {
        static CONFIG: OnceCell<AppConfig> = OnceCell::new();

        CONFIG.get_or_init(|| AppConfig {
            configure: Arc::new(Mutex::new(Configure::new())),
        })
    }

    pub fn latest() -> Arc<Mutex<Configure>> {
        Self::global().configure.clone()
    }
}
