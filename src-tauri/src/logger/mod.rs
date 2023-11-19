use std::env;

use log::LevelFilter;
use tauri::plugin::TauriPlugin;
use tauri::Runtime;
use tauri_plugin_log::LogTarget;

use crate::consts::LOG_LEVEL;
use crate::utils::dir::app_log_dir;

pub fn build_logger<R: Runtime>() -> TauriPlugin<R> {
    let log_level: String = env::var(LOG_LEVEL).unwrap_or_else(|_| String::from("INFO"));
    let log_level = match log_level.as_str() {
        "ERROR" => LevelFilter::Error,
        "WARN" => LevelFilter::Warn,
        "INFO" => LevelFilter::Info,
        "DEBUG" => LevelFilter::Debug,
        "TRACE" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    tauri_plugin_log::Builder::default()
        .level(log_level)
        .targets([LogTarget::Folder(app_log_dir().unwrap()), LogTarget::Stdout])
        .build()
}
