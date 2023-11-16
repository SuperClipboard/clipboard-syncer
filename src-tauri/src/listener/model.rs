use serde::{Deserialize, Serialize};

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

#[derive(Clone, Deserialize, Serialize, Debug)]
pub(crate) struct TapChangeClipboardFrontendMessage {
    pub(crate) content: String,
    pub(crate) data_type: String,
}
