use std::fs;
use std::fs::File;
use std::path::Path;
use std::sync::Mutex;

use anyhow::Result;
use lazy_static::lazy_static;
use rusqlite::{Connection, OpenFlags};

use crate::model::record::Record;
use crate::utils::dirs::app_data_dir;
use crate::utils::string_util;

const SQLITE_FILE: &str = "clipboard-syncer.sqlite";

lazy_static! {
    pub static ref SQLITE_CLIENT: Mutex<SqliteDB> = Mutex::new({
        SqliteDB::init();
        SqliteDB::new()
    });
}

pub struct SqliteDB {
    conn: Connection,
}

impl SqliteDB {
    pub fn new() -> Self {
        let data_dir = app_data_dir().unwrap().join(SQLITE_FILE);
        let c = Connection::open_with_flags(data_dir, OpenFlags::SQLITE_OPEN_READ_WRITE).unwrap();
        SqliteDB { conn: c }
    }

    pub fn init() {
        let data_dir = app_data_dir().unwrap().join(SQLITE_FILE);

        fs::create_dir_all(data_dir.parent().unwrap()).unwrap();
        if !Path::new(&data_dir).exists() {
            println!("Database file initialized!");
            File::create(&data_dir).unwrap();
        }

        let c = Connection::open_with_flags(data_dir, OpenFlags::SQLITE_OPEN_READ_WRITE).unwrap();
        c.execute(Record::create_table_sql(), ()).unwrap();
    }

    pub fn insert_record(&self, r: Record) -> Result<i64> {
        let sql = "insert into record (content,md5,create_time,is_favorite,data_type,content_preview) values (?1,?2,?3,?4,?5,?6)";
        let md5 = string_util::md5(r.content.as_str());
        let now = chrono::Local::now().timestamp_millis() as u64;
        let content_preview = r.content_preview.unwrap_or("".to_string());
        let _res = self.conn.execute(
            sql,
            (
                &r.content,
                md5,
                now,
                &r.is_favorite,
                &r.data_type,
                content_preview,
            ),
        )?;
        Ok(self.conn.last_insert_rowid())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        assert!(!SQLITE_CLIENT.lock().unwrap().conn.is_busy());
    }
}
