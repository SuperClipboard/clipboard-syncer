use crate::config::app_config::AppConfig;
use anyhow::{bail, Result};
use tauri::{App, GlobalShortcutManager, Manager};

use crate::consts::MAIN_WINDOW;

pub enum ShortcutKeymapEnum {
    ToggleWindow,
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

        manager.register(
            &Self::get_shortcut_hotkey_config(ShortcutKeymapEnum::ToggleWindow)?,
            move || {
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                };
            },
        )?;

        Ok(())
    }

    fn get_shortcut_hotkey_config(shortcut_key: ShortcutKeymapEnum) -> Result<String> {
        match shortcut_key {
            ShortcutKeymapEnum::ToggleWindow => {
                match AppConfig::latest().read().toggle_window_hotkey.clone() {
                    Some(hotkey) => Ok(hotkey),
                    None => {
                        bail!("toggle_window_hotkey not configured!");
                    }
                }
            }
        }
    }
}
