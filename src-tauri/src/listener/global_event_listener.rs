use anyhow::Result;
use tauri::{App, Manager};

#[derive(Debug)]
pub enum EventListenTypeEnum {
    ChangeClipBoard,
}

impl From<EventListenTypeEnum> for String {
    fn from(value: EventListenTypeEnum) -> Self {
        match value {
            EventListenTypeEnum::ChangeClipBoard => "cbs://change-clipboard-listen".into(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GlobalEventListener;

impl GlobalEventListener {
    pub fn register_all_global_listeners(app: &mut App) -> Result<()> {
        Self::change_clipboard_listener(app)
    }

    fn change_clipboard_listener(app: &mut App) -> Result<()> {
        app.listen_global(EventListenTypeEnum::ChangeClipBoard, |e| {
            println!("got {:?} with payload {:?}", EventListenTypeEnum::ChangeClipBoard, e.payload());
        });

        Ok(())
    }
}

