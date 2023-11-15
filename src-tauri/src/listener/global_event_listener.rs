//!
//! Global event listener listens the event emitted from the frontend!
//!
use anyhow::Result;
use log::info;
use tauri::{App, Manager};

#[derive(Debug)]
pub enum EventListenTypeEnum {
    TapChangeClipboardFrontend,
}

impl From<EventListenTypeEnum> for String {
    fn from(value: EventListenTypeEnum) -> Self {
        match value {
            EventListenTypeEnum::TapChangeClipboardFrontend => {
                "cbs://tap-change-clipboard-frontend".into()
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GlobalEventListener;

impl GlobalEventListener {
    pub fn register_all_global_listeners(app: &mut App) -> Result<()> {
        Self::tap_change_clipboard_frontend_listener(app)
    }

    fn tap_change_clipboard_frontend_listener(app: &mut App) -> Result<()> {
        app.listen_global(EventListenTypeEnum::TapChangeClipboardFrontend, |e| {
            info!(
                "got {:?} with payload {:?}",
                EventListenTypeEnum::TapChangeClipboardFrontend,
                e.payload()
            );
        });

        Ok(())
    }
}
