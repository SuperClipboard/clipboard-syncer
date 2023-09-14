use std::collections::HashSet;
use std::sync::OnceLock;

use log::{error, info};
use tokio::sync::Mutex;

use crate::consts::RECORD_LIMIT_THRESHOLD;
use crate::dao::record_cache_dao::RecordCacheDao;
use crate::models::record_cache::RecordCache;

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
        let d = match RecordCacheDao::list_all_record_caches_with_limit(RECORD_LIMIT_THRESHOLD) {
            Ok(data) => data,
            Err(e) => {
                error!("Load local cache failed: {:?}", e);
                HashSet::new()
            }
        };

        info!("load data success, size: {:#?}", d.len());

        Self { data: d }
    }

    pub fn contains(&self, k: &RecordCache) -> bool {
        match self.data.get(k) {
            None => false,
            Some(inner_data) => inner_data.create_time.ge(&k.create_time),
        }
    }

    pub fn get(&self, other_md5: &str) -> Option<RecordCache> {
        self.data
            .get(&RecordCache {
                md5: other_md5.to_string(),
                create_time: -1,
            })
            .cloned()
    }

    pub fn add(&mut self, data: RecordCache) -> bool {
        match self.contains(&data) {
            true => false,
            false => {
                self.data.replace(data);
                true
            }
        }
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

    pub fn calculate_diff(&self, data: &HashSet<RecordCache>) -> HashSet<RecordCache> {
        let mut diff = HashSet::new();

        data.iter().for_each(|item| match self.data.get(item) {
            None => {
                diff.insert(item.clone());
            }
            Some(inner_item) => {
                if inner_item.create_time < item.create_time {
                    diff.insert(item.clone());
                }
            }
        });

        diff
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
    use std::collections::HashSet;

    use crate::models::record_cache::RecordCache;
    use crate::storage::cache::CacheHandler;

    #[test]
    fn test_basic() {
        let mut s = CacheHandler::global().blocking_lock();
        s.clear();
        assert_eq!(s.data.len(), 0);

        s.add(RecordCache {
            md5: "1".to_string(),
            create_time: 3,
        });
        assert_eq!(s.data.len(), 1);

        s.add(RecordCache {
            md5: "1".to_string(),
            create_time: 1,
        });
        assert_eq!(s.data.len(), 1);

        s.remove(&RecordCache {
            md5: "1".to_string(),
            create_time: -1,
        });
        assert_eq!(s.data.len(), 0);

        s.clear();
    }

    #[test]
    fn test_add1() {
        let mut s = CacheHandler::global().blocking_lock();
        s.clear();

        s.add(RecordCache {
            md5: "1".to_string(),
            create_time: 1,
        });
        s.print();
        assert_eq!(s.get("1").unwrap().create_time, 1);

        s.add(RecordCache {
            md5: "1".to_string(),
            create_time: 3,
        });
        s.print();
        assert_eq!(s.get("1").unwrap().create_time, 3);

        s.clear();
    }

    #[test]
    fn test_add2() {
        let mut s = CacheHandler::global().blocking_lock();
        s.clear();

        s.add(RecordCache {
            md5: "1".to_string(),
            create_time: 3,
        });
        s.print();
        assert_eq!(s.get("1").unwrap().create_time, 3);

        s.add(RecordCache {
            md5: "1".to_string(),
            create_time: 1,
        });
        s.print();
        assert_eq!(s.get("1").unwrap().create_time, 3);

        s.clear();
    }

    #[test]
    fn test_merge() {
        let mut s = CacheHandler::global().blocking_lock();
        s.clear();

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
