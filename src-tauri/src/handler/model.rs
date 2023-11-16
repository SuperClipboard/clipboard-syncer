use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug)]
pub enum MessageTypeEnum {
    ChangeClipboardBackend,
}

impl From<MessageTypeEnum> for String {
    fn from(value: MessageTypeEnum) -> Self {
        match value {
            MessageTypeEnum::ChangeClipboardBackend => "cbs://change-clipboard-backend".into(),
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub struct Payload<M: Serialize + Clone + Debug> {
    pub(crate) message: M,
}
