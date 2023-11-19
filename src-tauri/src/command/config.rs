use log::error;

use crate::config::app_config::AppConfig;
use crate::config::configure::Configure;

#[tauri::command]
pub fn graphql_endpoint() -> Result<String, String> {
    match AppConfig::latest().read().graphql_port {
        None => Err(String::from("GraphQL endpoint config not found")),
        Some(graphql_port) => Ok(format!("http://localhost:{}/graphql", graphql_port)),
    }
}

#[tauri::command]
pub fn load_app_config() -> Result<Configure, String> {
    let config = AppConfig::latest().read().clone();
    Ok(config)
}

#[tauri::command]
pub async fn save_app_config(config: Configure) -> Result<(), String> {
    match AppConfig::modify_config(config).await {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("Call modify_config err: {}", err);
            Err("修改配置失败，请重试！".to_string())
        }
    }
}
