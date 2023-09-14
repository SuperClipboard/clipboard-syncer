use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::sync_proto::SyncData;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordCache {
    pub md5: String,
    pub create_time: i32,
}

impl From<RecordCache> for SyncData {
    fn from(val: RecordCache) -> Self {
        SyncData {
            md5: val.md5,
            create_time: val.create_time,
        }
    }
}

impl From<SyncData> for RecordCache {
    fn from(value: SyncData) -> Self {
        RecordCache {
            md5: value.md5,
            create_time: value.create_time,
        }
    }
}

impl Hash for RecordCache {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.md5.hash(state)
    }
}

impl Eq for RecordCache {}

impl PartialEq<Self> for RecordCache {
    fn eq(&self, other: &Self) -> bool {
        self.md5.eq(&other.md5)
    }
}

#[cfg(test)]
mod test {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;

    use crate::models::record_cache::RecordCache;

    #[test]
    fn test_eq() {
        assert_eq!(
            RecordCache {
                md5: "1".to_string(),
                create_time: 22,
            },
            RecordCache {
                md5: "1".to_string(),
                create_time: 1,
            }
        )
    }

    #[test]
    fn test_hash() {
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        assert_eq!(
            RecordCache {
                md5: "1".to_string(),
                create_time: 22,
            }
            .hash(&mut hasher1),
            RecordCache {
                md5: "1".to_string(),
                create_time: 1,
            }
            .hash(&mut hasher2)
        )
    }
}
