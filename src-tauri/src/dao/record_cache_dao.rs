use std::collections::HashSet;

use anyhow::Result;
use diesel::{QueryDsl, RunQueryDsl};

use crate::models::record_cache::RecordCache;
use crate::schema::t_record::dsl::*;
use crate::storage::sqlite::SQLITE_CLIENT;

pub struct RecordCacheDao;

impl RecordCacheDao {
    pub fn list_all_record_caches_with_limit(limit: usize) -> Result<HashSet<RecordCache>> {
        let c = &mut SQLITE_CLIENT.lock().unwrap().conn;

        let res = t_record
            .select((md5, create_time))
            .limit(limit as i64)
            .load::<RecordCache>(c)?;

        Ok(res.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::dao::record_cache_dao::RecordCacheDao;

    #[test]
    fn test_list_all_record_caches_with_limit() {
        let res = RecordCacheDao::list_all_record_caches_with_limit(1).unwrap();
        println!("list_all_record_caches_with_limit: {:?}", res);
        assert!(res.len() <= 1);
    }
}
