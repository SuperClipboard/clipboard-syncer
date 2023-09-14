use std::collections::HashSet;
use std::sync::OnceLock;

use crate::models::record_cache::RecordCache;
use log::info;
use parking_lot::Mutex;

#[derive(Debug)]
pub struct CacheHandler {
    data: HashSet<RecordCache>,
}

impl CacheHandler {
    // init global
    pub fn global() -> &'static Mutex<CacheHandler> {
        static STORAGE: OnceLock<Mutex<CacheHandler>> = OnceLock::new();

        STORAGE.get_or_init(|| Mutex::new(CacheHandler::new()))
    }

    fn new() -> Self {
        let d = HashSet::new();

        info!("load data success: {:#?}", d);

        Self { data: d }
    }

    pub fn contains(&self, k: &RecordCache) -> bool {
        self.data.contains(k)
    }

    pub fn add(&mut self, data: RecordCache) {
        self.data.insert(data);
    }

    pub fn remove(&mut self, k: &RecordCache) -> bool {
        self.data.remove(k)
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn merge_data(&mut self, data: &HashSet<RecordCache>) {
        data.iter().for_each(|item| match self.data.get(item) {
            None => {
                self.data.insert(item.clone());
            }
            Some(inner_item) => {
                if inner_item.create_time < item.create_time {
                    self.data.replace(item.clone());
                }
            }
        })
    }

    pub fn get_copy_data(&self) -> HashSet<RecordCache> {
        self.data.clone()
    }

    pub fn print(&self) {
        println!("{:#?}", self.data);
    }
}

#[cfg(test)]
mod tests {
    use crate::models::record_cache::RecordCache;
    use crate::storage::cache::CacheHandler;
    use std::collections::HashSet;

    #[test]
    fn test_main() {
        let mut s = CacheHandler::global().lock();
        s.print();
        assert_eq!(s.data.len(), 0);

        s.add(RecordCache {
            md5: "1".to_string(),
            create_time: 3,
        });
        s.print();
        assert_eq!(s.data.len(), 1);

        s.add(RecordCache {
            md5: "1".to_string(),
            create_time: 1,
        });
        s.print();
        assert_eq!(s.data.len(), 1);

        s.remove(&RecordCache {
            md5: "1".to_string(),
            create_time: 1,
        });
        s.print();
        assert_eq!(s.data.len(), 0);

        s.clear();
    }

    #[test]
    fn test_merge() {
        let mut s = CacheHandler::global().lock();
        s.add(RecordCache {
            md5: "1".to_string(),
            create_time: 0,
        });

        let s2 = HashSet::from([
            RecordCache {
                md5: "1".to_string(),
                create_time: 10,
            },
            RecordCache {
                md5: "2".to_string(),
                create_time: 2,
            },
        ]);

        s.merge_data(&s2);
        let merged = s.get_copy_data();

        assert_eq!(merged.len(), 2);
        assert_eq!(
            merged
                .get(&RecordCache {
                    md5: "1".to_string(),
                    create_time: -1,
                })
                .unwrap()
                .create_time,
            10
        );
        assert_eq!(
            merged
                .get(&RecordCache {
                    md5: "2".to_string(),
                    create_time: -1,
                })
                .unwrap()
                .create_time,
            2
        );

        s.clear();
    }
}
