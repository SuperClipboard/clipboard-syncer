#[derive(serde::Serialize, serde::Deserialize, Debug, Default, PartialEq)]
pub struct Record {
    pub id: u64,
    pub content: String,
    pub content_preview: Option<String>,
    // data_type(文本=text、图片=image)
    pub data_type: String,
    pub md5: String,
    pub create_time: u64,
    pub is_favorite: bool,
    pub tags: String,
}

impl Record {
    pub fn create_table_sql() -> &'static str {
        r#"
        create table if not exists t_record (
            id          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            content     TEXT,
            content_preview     TEXT,
            data_type   VARCHAR(20) DEFAULT '',
            md5         VARCHAR(200) DEFAULT '',
            create_time INTEGER,
            is_favorite INTEGER DEFAULT 0,
            tags        VARCHAR(256) DEFAULT ''
        );
        "#
    }
}
