use local_ip_address::local_ip;
use log::{debug, error, info, warn};

use crate::dao::record_dao::RecordDao;
use crate::handler::global_handler::GlobalHandler;
use crate::handler::model::MessageTypeEnum;
use crate::models::image_data::ImageData;
use crate::models::record;
use crate::models::record::DataTypeEnum;
use crate::p2panda::graphql::{field, GraphQLHandler};
use crate::utils::clipboard::ClipBoardOperator;
use crate::utils::json;
use crate::utils::string::base64_decode;

#[tauri::command]
pub async fn tap_change_clipboard(content: String, data_type: String) -> Result<(), String> {
    debug!("got content: {:?} with data_type {:?}", content, data_type);

    if data_type.eq(&String::from(DataTypeEnum::TEXT)) {
        let raw_content = base64_decode(&content);
        let content = match String::from_utf8(raw_content) {
            Ok(content) => content,
            Err(err) => {
                error!(
                    "Decode base64 data failed, content: {:?}, err: {}",
                    content, err
                );
                return Err(err.to_string());
            }
        };

        if let Err(e) = ClipBoardOperator::set_text(content) {
            let err_msg = format!("Set text to clipboard err: {}", e);
            error!("{}", err_msg);
            return Err(err_msg);
        };
    } else if data_type.eq(&String::from(DataTypeEnum::IMAGE)) {
        let raw_image = base64_decode(&content);
        let content = match String::from_utf8(raw_image) {
            Ok(content) => content,
            Err(err) => {
                error!("Decode base64 data failed, err: {}", err);
                return Err(err.to_string());
            }
        };
        let image_data = match json::parse::<ImageData>(&content) {
            Ok(image_data) => image_data,
            Err(err) => {
                let err_msg = format!("Parse image data failed, err: {}", err);
                error!("{}", err_msg);
                return Err(err_msg);
            }
        };

        if let Err(e) = ClipBoardOperator::set_image(image_data) {
            let err_msg = format!("Set image to clipboard err: {}", e);
            error!("{}", err_msg);
            return Err(err_msg);
        };
    } else {
        warn!("Unknown data type for the record: {}", data_type)
    };

    Ok(())
}

#[tauri::command]
pub async fn delete_record(view_id: String) -> Result<(), String> {
    debug!("got view_id {:?}", view_id);

    let handler = &mut GraphQLHandler::global().lock().await;
    let delete_res = handler.delete_instance(record::SCHEMA_ID, &view_id).await;
    if delete_res.is_err() {
        error!("call delete instance error: {:?}", delete_res.unwrap_err())
    }

    if let Err(e) = GlobalHandler::push_message_to_window(
        MessageTypeEnum::DeleteClipboardRecordBackend,
        format!("delete view_id: {} success", view_id),
    ) {
        error!("send DeleteClipboardRecordBackend message err: {:?}", e)
    };

    Ok(())
}

#[tauri::command]
pub async fn toggle_favorite_record(view_id: String, old_favorite: i32) -> Result<(), String> {
    let favorite = if old_favorite.ne(&0) { 0 } else { 1 };

    match RecordDao::update_record_with_fields(
        &view_id,
        &mut [
            field("is_favorite", &favorite.to_string()),
            field("latest_addr", &local_ip().unwrap().to_string()),
        ],
    )
    .await
    {
        Ok(opt_id) => {
            info!(
                "call update_record_with_fields successfully: opt_id={}",
                opt_id
            )
        }
        Err(err) => {
            let err_msg = format!("call delete instance error: {:?}", err);
            error!("call update_record_with_fields error: {:?}", err);
            return Err(err_msg);
        }
    };

    if let Err(e) = GlobalHandler::push_message_to_window(
        MessageTypeEnum::UpdateClipboardRecordBackend,
        format!("update favorite record success, view_id: {}", view_id),
    ) {
        error!("send UpdateClipboardRecordBackend message err: {:?}", e)
    };

    Ok(())
}
