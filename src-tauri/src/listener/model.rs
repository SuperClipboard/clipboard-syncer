use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum EventListenTypeEnum {
    TapChangeClipboardFrontend,
    DeleteClipboardRecordFrontend,
}

impl From<EventListenTypeEnum> for String {
    fn from(value: EventListenTypeEnum) -> Self {
        match value {
            EventListenTypeEnum::TapChangeClipboardFrontend => {
                "cbs://tap-change-clipboard-frontend".into()
            }
            EventListenTypeEnum::DeleteClipboardRecordFrontend => {
                "cbs://delete-clipboard-record-frontend".into()
            }
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub(crate) struct TapChangeClipboardFrontendMessage {
    pub(crate) content: String,
    pub(crate) data_type: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub(crate) struct DeleteClipboardRecordFrontendMessage {
    pub(crate) view_id: String,
}
