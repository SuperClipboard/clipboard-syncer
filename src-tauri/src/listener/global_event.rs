use anyhow::Result;
use tauri::App;

#[derive(Debug, Default)]
pub struct GlobalEventListener;

impl GlobalEventListener {
    pub fn register_all_global_listeners(_app: &mut App) -> Result<()> {
        Ok(())
    }
}
