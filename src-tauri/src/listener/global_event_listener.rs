//!
//! Global event listener listens the event emitted from the frontend!
//!
use anyhow::Result;
use log::{debug, error, warn};
use tauri::{App, Manager};

use crate::handler::global_handler::GlobalHandler;
use crate::listener::model::{
    DeleteClipboardRecordFrontendMessage, EventListenTypeEnum, TapChangeClipboardFrontendMessage,
};
use crate::models::image_data::ImageData;
use crate::models::record;
use crate::models::record::DataTypeEnum;
use crate::p2panda::graphql::GraphQLHandler;
use crate::utils::clipboard::ClipBoardOperator;
use crate::utils::json;
use crate::utils::string::base64_decode;

#[derive(Debug, Default, Clone)]
pub struct GlobalEventListener;

impl GlobalEventListener {
    pub fn register_all_global_listeners(app: &mut App) -> Result<()> {
        Self::tap_change_clipboard_frontend_listener(app)?;
        Self::delete_clipboard_record_listener(app)?;
        Ok(())
    }

    fn tap_change_clipboard_frontend_listener(app: &mut App) -> Result<()> {
        app.listen_global(EventListenTypeEnum::TapChangeClipboardFrontend, |e| {
            debug!(
                "got {:?} with payload {:?}",
                EventListenTypeEnum::TapChangeClipboardFrontend,
                e.payload()
            );

            let record = match e.payload() {
                Some(payload) => {
                    match serde_json::from_str::<TapChangeClipboardFrontendMessage>(payload) {
                        Ok(record) => record,
                        Err(err) => {
                            warn!(
                                "Parse {:?} event payload failed: {}",
                                EventListenTypeEnum::TapChangeClipboardFrontend,
                                err
                            );
                            return;
                        }
                    }
                }
                None => {
                    warn!(
                        "{:?} event payload is empty",
                        EventListenTypeEnum::TapChangeClipboardFrontend
                    );
                    return;
                }
            };

            if record.data_type.eq(&String::from(DataTypeEnum::TEXT)) {
                let raw_content = base64_decode(&record.content);
                let content = match String::from_utf8(raw_content) {
                    Ok(content) => content,
                    Err(err) => {
                        error!(
                            "Decode base64 data failed, content: {:?}, err: {}",
                            record.content, err
                        );
                        return;
                    }
                };

                if let Err(e) = ClipBoardOperator::set_text(content) {
                    error!("Set text to clipboard err: {}", e);
                };
            } else if record.data_type.eq(&String::from(DataTypeEnum::IMAGE)) {
                let raw_image = base64_decode(&record.content);
                let content = match String::from_utf8(raw_image) {
                    Ok(content) => content,
                    Err(err) => {
                        error!(
                            "Decode base64 data failed, content: {:?}, err: {}",
                            record.content, err
                        );
                        return;
                    }
                };
                let image_data = match json::parse::<ImageData>(&content) {
                    Ok(image_data) => image_data,
                    Err(err) => {
                        error!("Parse image data failed, err: {}", err);
                        return;
                    }
                };

                if let Err(e) = ClipBoardOperator::set_image(image_data) {
                    error!("Set image to clipboard err: {}", e);
                };
            } else {
                warn!("Unknown data type for the record: {}", record.data_type)
            };
        });

        Ok(())
    }

    fn delete_clipboard_record_listener(app: &mut App) -> Result<()> {
        app.listen_global(EventListenTypeEnum::DeleteClipboardRecordFrontend, |e| {
            debug!(
                "got {:?} with payload {:?}",
                EventListenTypeEnum::DeleteClipboardRecordFrontend,
                e.payload()
            );

            let msg = match e.payload() {
                Some(payload) => {
                    match serde_json::from_str::<DeleteClipboardRecordFrontendMessage>(payload) {
                        Ok(record) => record,
                        Err(err) => {
                            warn!(
                                "Parse {:?} event payload failed: {}",
                                EventListenTypeEnum::DeleteClipboardRecordFrontend,
                                err
                            );
                            return;
                        }
                    }
                }
                None => {
                    warn!(
                        "{:?} event payload is empty",
                        EventListenTypeEnum::DeleteClipboardRecordFrontend
                    );
                    return;
                }
            };

            tokio::task::spawn(async move {
                let handler = &mut GraphQLHandler::global().lock().await;
                let delete_res = handler
                    .delete_instance(record::SCHEMA_ID, &msg.view_id)
                    .await;
                if delete_res.is_err() {
                    error!("call delete instance error: {:?}", delete_res.unwrap_err())
                }

                if let Err(e) = GlobalHandler::change_clipboard_backend_handler(format!(
                    "delete view_id: {} success",
                    msg.view_id
                )) {
                    error!("send change_clipboard_backend message err: {:?}", e)
                };
            });
        });

        Ok(())
    }
}
