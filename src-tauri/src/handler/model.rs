use std::fmt::Debug;

use serde::Serialize;

#[derive(Debug)]
pub enum MessageTypeEnum {
    ChangeClipboardBackend,
    UpdateClipboardRecordBackend,
    DeleteClipboardRecordBackend,
}

impl From<MessageTypeEnum> for &'static str {
    fn from(value: MessageTypeEnum) -> Self {
        match value {
            MessageTypeEnum::ChangeClipboardBackend => "cbs://change-clipboard-backend",
            MessageTypeEnum::UpdateClipboardRecordBackend => {
                "cbs://update-clipboard-record-backend"
            }
            MessageTypeEnum::DeleteClipboardRecordBackend => {
                "cbs://delete-clipboard-record-backend"
            }
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub struct Payload<M: Serialize + Clone + Debug> {
    pub(crate) message: M,
}
