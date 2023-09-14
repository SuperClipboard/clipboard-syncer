use crate::consts::RECORD_LIMIT_THRESHOLD;
use anyhow::Result;
use diesel::associations::HasTable;
use diesel::dsl::count_star;
use diesel::sql_types::Integer;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use log::debug;

use crate::models::record::Record;
use crate::schema::t_record::dsl::*;
use crate::storage::sqlite::SQLITE_CLIENT;
use crate::utils::string;

pub struct RecordDao;

impl RecordDao {
    pub fn insert_if_not_exist(mut r: Record) -> Result<()> {
        let now = chrono::Local::now().timestamp() as i32;
        let mo5_str = string::md5(r.content.as_str());
        r.md5 = mo5_str.clone();
        r.create_time = now;

        let c = &mut SQLITE_CLIENT.lock().unwrap().conn;
        let res = t_record
            .filter(md5.eq(mo5_str))
            .limit(1)
            .load::<Record>(c)?;

        match res.len() {
            // no record
            0 => {
                diesel::insert_into(t_record::table())
                    .values(&r)
                    .execute(c)?;

                debug!("insert new record successfully: {:?}", r)
            }
            // find record
            _ => {
                Self::update_record_create_time(c, res[0].id.unwrap_or(0), now)?;

                debug!("update record successfully: {:?}", r)
            }
        };
        Ok(())
    }

    // Delete record if over limit
    pub fn delete_record_with_limit(limit: usize) -> Result<bool> {
        let c = &mut SQLITE_CLIENT.lock().unwrap().conn;

        // 先查询count，如果count - limit > RECORD_LIMIT_THRESHOLD 才删除超出limit部分记录，防止频繁操作数据库
        let cnt = t_record
            .select(count_star())
            .filter(is_favorite.eq(0))
            .get_result::<i64>(c)? as usize;

        // Not reach the threshold
        if cnt < RECORD_LIMIT_THRESHOLD + limit {
            return Ok(false);
        }

        let actual_remove_cnt = (cnt - limit) as i32;
        diesel::sql_query(
            r#"SELECT * FROM t_record
        WHERE is_favorite = 0
        order by create_time asc
        LIMIT ?"#,
        )
        .bind::<Integer, _>(actual_remove_cnt)
        .execute(c)?;

        Ok(true)
    }

    pub fn find_records_in_md5_list(md5_list: &Vec<String>) -> Result<Vec<Record>> {
        let c = &mut SQLITE_CLIENT.lock().unwrap().conn;

        let res = t_record.filter(md5.eq_any(md5_list)).load::<Record>(c)?;

        Ok(res)
    }

    fn update_record_create_time(c: &mut SqliteConnection, aid: i32, now: i32) -> Result<()> {
        let _ = diesel::update(t_record.filter(id.eq(aid)))
            .set(create_time.eq(now))
            .execute(c)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::dao::record_dao::RecordDao;
    use crate::models::record;
    use crate::models::record::Record;

    #[test]
    fn test_insert_if_not_exist() {
        let now = chrono::Local::now().timestamp() as i32;
        RecordDao::insert_if_not_exist(Record {
            content: "abc".to_string(),
            content_preview: Some("abc".to_string()),
            data_type: record::DataTypeEnum::TEXT.into(),
            create_time: now,
            ..Default::default()
        })
        .unwrap();
    }

    #[test]
    fn test_find_records_in_md5_list() {
        let res = RecordDao::find_records_in_md5_list(&vec![
            "722d70d1dbae5a52b68803e48d442bce".to_string(),
            "c48951707c41961160dbdba285b9864a".to_string(),
        ])
        .unwrap();

        println!("{:#?}", res);
    }
}
