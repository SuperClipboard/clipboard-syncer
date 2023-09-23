use crate::dao::record_dao::RecordDao;
use crate::models::record::Record;

#[tauri::command]
pub fn find_records_by_pages(limit: usize, offset: usize) -> Result<Vec<Record>, String> {
    match RecordDao::find_records_by_pages(limit, offset) {
        Ok(res) => Ok(res),
        Err(e) => Err(e.to_string()),
    }
}
