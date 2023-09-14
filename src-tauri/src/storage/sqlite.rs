use std::fs;
use std::fs::File;
use std::path::Path;
use std::sync::Mutex;

use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use lazy_static::lazy_static;
use log::info;

use crate::utils::dir::app_data_dir;

const SQLITE_FILE: &str = "clipboard-syncer.sqlite";

lazy_static! {
    pub static ref SQLITE_CLIENT: Mutex<SqliteDB> = Mutex::new({
        SqliteDB::init();
        SqliteDB::new()
    });
}

pub struct SqliteDB {
    pub conn: SqliteConnection,
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

impl Default for SqliteDB {
    fn default() -> Self {
        Self::init();
        Self::new()
    }
}

impl SqliteDB {
    pub fn new() -> Self {
        let data_dir = app_data_dir().unwrap().join(SQLITE_FILE);
        let db_uri = format!("sqlite://{}", data_dir.to_str().unwrap());
        let c = SqliteConnection::establish(&db_uri)
            .unwrap_or_else(|_| panic!("Error connecting to {:?}", db_uri));
        SqliteDB { conn: c }
    }

    pub fn init() {
        let data_dir = app_data_dir().unwrap().join(SQLITE_FILE);

        fs::create_dir_all(data_dir.parent().unwrap()).unwrap();
        if !Path::new(&data_dir).exists() {
            info!("Database file initialized!");
            File::create(&data_dir).unwrap();
        }

        let db_uri = format!("sqlite://{}", data_dir.to_str().unwrap());
        let mut c = SqliteConnection::establish(&db_uri)
            .unwrap_or_else(|_| panic!("Error connecting to {:?}", db_uri));

        // This will run the necessary migrations.
        c.run_pending_migrations(MIGRATIONS)
            .unwrap_or_else(|_| panic!("Error running migration"));

        info!("Database migration success!");
    }

    // pub fn insert_record(&self, r: Record) -> Result<i64> {
    //     let sql = "insert into record (content,md5,create_time,is_favorite,data_type,content_preview) values (?1,?2,?3,?4,?5,?6)";
    //     let md5 = string_util::md5(r.content.as_str());
    //     let now = chrono::Local::now().timestamp() as u64;
    //     let content_preview = r.content_preview.unwrap_or("".to_string());
    //     let _res = self.conn.execute(
    //         sql,
    //         (
    //             &r.content,
    //             md5,
    //             now,
    //             &r.is_favorite,
    //             &r.data_type,
    //             content_preview,
    //         ),
    //     )?;
    //     Ok(self.conn.last_insert_rowid())
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_build() {}
}
