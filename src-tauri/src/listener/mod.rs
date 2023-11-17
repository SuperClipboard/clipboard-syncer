use anyhow::Result;
use tauri::App;

use crate::listener::clipboard::ClipboardListener;
use crate::listener::global_event::GlobalEventListener;
use crate::listener::shortcut::ShortcutListener;

mod clipboard;
mod global_event;
mod shortcut;

pub fn register_all_listeners(app: &mut App) -> Result<()> {
    // Start global application listener
    GlobalEventListener::register_all_global_listeners(app)?;

    // Start listening for clipboard
    ClipboardListener::listen();

    // Start shortcut listener
    if cfg!(not(target_os="linux")) {
        ShortcutListener::register_all_hotkey_listeners(app)?;
    }

    Ok(())
}
