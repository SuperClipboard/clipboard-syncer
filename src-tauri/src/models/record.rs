use diesel::prelude::*;
use diesel::{Queryable, Selectable};

#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Default,
    PartialEq,
    Queryable,
    Selectable,
    Insertable,
)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::schema::t_record)]
pub struct Record {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub content: String,
    pub content_preview: Option<String>,
    // data_type(文本=text、图片=image)
    pub data_type: String,
    pub md5: String,
    pub create_time: i32,
    pub is_favorite: i32,
    pub tags: String,
}

pub enum DataTypeEnum {
    TEXT,
    IMAGE,
}

impl From<DataTypeEnum> for String {
    fn from(value: DataTypeEnum) -> Self {
        match value {
            DataTypeEnum::TEXT => "text".into(),
            DataTypeEnum::IMAGE => "image".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::t_record::dsl::t_record;
    use crate::storage::sqlite::SQLITE_CLIENT;

    use super::*;

    #[test]
    fn test_select_all() {
        let c = &mut SQLITE_CLIENT.lock().unwrap().conn;
        let res = t_record.get_results::<Record>(c);
        println!("{:#?}", res);
    }
}
