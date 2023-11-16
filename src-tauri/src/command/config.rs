use crate::config::app_config::AppConfig;

#[tauri::command]
pub fn graphql_endpoint() -> Result<String, String> {
    match AppConfig::latest().read().graphql_port.clone() {
        None => Err(String::from("GraphQL endpoint config not found")),
        Some(graphql_port) => Ok(format!("http://localhost:{}/graphql", graphql_port)),
    }
}
