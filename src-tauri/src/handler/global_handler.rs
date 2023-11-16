//!
//! Global handler emit the events from backend!
//!
use std::fmt::Debug;
use std::sync::Arc;

use anyhow::{bail, Result};
use log::{error, info};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use serde::Serialize;
use tauri::{AppHandle, Manager};

use crate::consts::MAIN_WINDOW;
use crate::handler::model::{MessageTypeEnum, Payload};

#[derive(Debug, Default, Clone)]
pub struct GlobalHandler {
    pub app_handle: Arc<Mutex<Option<AppHandle>>>,
}

impl GlobalHandler {
    pub fn global() -> &'static GlobalHandler {
        static HANDLE: OnceCell<GlobalHandler> = OnceCell::new();

        HANDLE.get_or_init(|| GlobalHandler {
            app_handle: Arc::new(Mutex::new(None)),
        })
    }

    pub fn init(&self, app_handle: AppHandle) {
        *self.app_handle.lock() = Some(app_handle);
    }

    pub fn push_message_to_window<M: Serialize + Clone + Debug>(
        msg_type: MessageTypeEnum,
        msg: M,
    ) -> Result<()> {
        match msg_type {
            MessageTypeEnum::ChangeClipboardBackend => {
                info!("send ChangeClipBoard message: {:?}", msg);
                Self::change_clipboard_backend_handler(msg)
            }
        }
    }

    pub fn change_clipboard_backend_handler<M: Serialize + Clone + Debug>(msg: M) -> Result<()> {
        let app_handle = Self::global().app_handle.lock();
        if app_handle.is_none() {
            error!(
                "Cannot push message to window: {:?}, {:?}",
                MessageTypeEnum::ChangeClipboardBackend,
                msg
            );
            bail!("application not initiated, push_message_to_window error");
        }

        let window = app_handle
            .as_ref()
            .unwrap()
            .get_window(MAIN_WINDOW)
            .unwrap();
        window.emit_all(
            &String::from(MessageTypeEnum::ChangeClipboardBackend),
            Payload { message: msg },
        )?;

        Ok(())
    }
}
