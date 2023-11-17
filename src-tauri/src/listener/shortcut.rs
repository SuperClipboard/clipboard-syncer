use anyhow::Result;
use tauri::{App, GlobalShortcutManager, Manager};

use crate::consts::MAIN_WINDOW;

pub enum ShortcutKeymapEnum {
    ToggleWindow,
}

impl From<ShortcutKeymapEnum> for &'static str {
    fn from(value: ShortcutKeymapEnum) -> Self {
        match value {
            ShortcutKeymapEnum::ToggleWindow => "Cmd+k",
        }
    }
}

#[derive(Debug, Default)]
pub struct ShortcutListener;

impl ShortcutListener {
    pub fn register_all_hotkey_listeners(app: &mut App) -> Result<()> {
        Self::register_toggle_window(app)?;

        Ok(())
    }

    fn register_toggle_window(app: &mut App) -> Result<()> {
        let app_handle = app.handle();
        let mut manager = app_handle.global_shortcut_manager();
        let window = app_handle.get_window(MAIN_WINDOW).unwrap();

        manager.register(ShortcutKeymapEnum::ToggleWindow.into(), move || {
            if window.is_visible().unwrap() {
                window.hide().unwrap();
            } else {
                window.show().unwrap();
            };
        })?;

        Ok(())
    }
}
