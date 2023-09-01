use diesel::{Queryable, Selectable};
use diesel::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, PartialEq, Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = crate::schema::t_record)]
pub struct Record {
    pub id: i32,
    pub content: String,
    pub content_preview: Option<String>,
    // data_type(文本=text、图片=image)
    pub data_type: String,
    pub md5: String,
    pub create_time: i32,
    pub is_favorite: i32,
    pub tags: String,
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
        println!("{:?}", res);
    }
}
