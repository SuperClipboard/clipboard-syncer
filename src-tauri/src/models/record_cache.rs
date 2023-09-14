use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

    #[test]
    fn test_order() {

    }
}
