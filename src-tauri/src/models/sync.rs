use crate::sync_proto::{SyncMeta, SyncRecord};

impl From<SyncMeta> for SyncRecord {
    fn from(value: SyncMeta) -> Self {
        SyncRecord {
            md5: value.md5,
            create_time: value.create_time,
            content: "".to_string(),
            content_preview: None,
            data_type: "".to_string(),
            is_favorite: 0,
            tags: "".to_string(),
            latest_addr: "".to_string(),
        }
    }
}

impl From<SyncRecord> for SyncMeta {
    fn from(value: SyncRecord) -> Self {
        SyncMeta {
            md5: value.md5,
            create_time: value.create_time,
        }
    }
}
