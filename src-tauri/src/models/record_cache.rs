use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RecordCache {
    md5: String,
    create_time: i32,
}

impl Eq for RecordCache {}

impl PartialEq<Self> for RecordCache {
    fn eq(&self, other: &Self) -> bool {
        self.md5.eq(&other.md5)
    }
}

impl PartialOrd<Self> for RecordCache {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::from(self.create_time.cmp(&other.create_time))
    }
}

impl Ord for RecordCache {
    fn cmp(&self, other: &Self) -> Ordering {
        self.create_time.cmp(&other.create_time)
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;
    use crate::models::record_cache::RecordCache;

    #[test]
    fn test_order() {
        let s = BTreeSet::from([
            RecordCache {
                md5: "3".to_string(),
                create_time: 1,
            },
            RecordCache {
                md5: "1".to_string(),
                create_time: 3,
            },
            RecordCache {
                md5: "2".to_string(),
                create_time: 2,
            },
        ]);

        let mut i = 1;
        s.iter().for_each(|item| {
            println!("{:?}", item);
            assert_eq!(item.create_time, i);
            i += 1;
        })
    }
}
