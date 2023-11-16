//!
//! Global event listener listens the event emitted from the frontend!
//!
use anyhow::Result;
use log::{debug, error, warn};
use tauri::{App, Manager};

use crate::listener::model::{EventListenTypeEnum, TapChangeClipboardFrontendMessage};
use crate::models::image_data::ImageData;
use crate::models::record::DataTypeEnum;
use crate::utils::clipboard::ClipBoardOperator;
use crate::utils::json;
use crate::utils::string::base64_decode;

#[derive(Debug, Default, Clone)]
pub struct GlobalEventListener;

impl GlobalEventListener {
    pub fn register_all_global_listeners(app: &mut App) -> Result<()> {
        Self::tap_change_clipboard_frontend_listener(app)
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
}
