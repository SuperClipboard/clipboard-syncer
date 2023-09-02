use std::thread;

use arboard::Clipboard;
use chrono::Duration;
use log::{error, info};

use crate::config::app_config::AppConfig;
use crate::dao::record_dao::RecordDao;
use crate::handler::{global_handler, MessageTypeEnum};
use crate::models::image_data::ImageData;
use crate::models::record;
use crate::models::record::Record;
use crate::utils::{image, json, string};

pub struct ClipboardListener;

impl ClipboardListener {
    // Check clipboard content in each 1 second
    const WAIT_MILLIS: i64 = 1000;

    const TEXT_PREVIEW_LEN: usize = 100;

    pub fn listen() {
        tauri::async_runtime::spawn(async {
            let mut last_content_md5 = String::new();
            let mut last_img_md5 = String::new();
            let mut clipboard = Clipboard::new().unwrap();
            info!("start clipboard listener");

            loop {
                let mut need_notify = false;
                let text = clipboard.get_text();
                let _ = text.map(|text| {
                    let content_origin = text.clone();
                    let content = text.trim();
                    let md5 = string::md5(&content_origin);
                    if !content.is_empty() && md5 != last_content_md5 {
                        // Has new clip contents
                        let content_preview = if content.len() > Self::TEXT_PREVIEW_LEN {
                            Some(content.chars().take(Self::TEXT_PREVIEW_LEN).collect())
                        } else {
                            Some(content.to_string())
                        };

                        let res = RecordDao::insert_if_not_exist(Record {
                            content: content_origin,
                            content_preview,
                            data_type: record::DataTypeEnum::TEXT.into(),
                            ..Default::default()
                        });
                        match res {
                            Ok(_) => {
                                need_notify = true;
                            }
                            Err(e) => {
                                error!("insert record error: {}", e);
                            }
                        }
                        last_content_md5 = md5;
                    }
                });

                let img = clipboard.get_image();
                let _ = img.map(|img| {
                    let img_md5 = string::md5_by_bytes(&img.bytes);
                    if img_md5 != last_img_md5 {
                        // 有新图片产生
                        let base64 = image::rgba8_to_base64(&img);
                        let content_db = ImageData {
                            width: img.width,
                            height: img.height,
                            base64,
                        };
                        // 压缩画质作为预览图，防止渲染时非常卡顿
                        let jpeg_base64 = image::rgba8_to_jpeg_base64(&img, 75);
                        let content_preview_db = ImageData {
                            width: img.width,
                            height: img.height,
                            base64: jpeg_base64,
                        };
                        let content = json::stringify(&content_db).unwrap();
                        let content_preview = json::stringify(&content_preview_db).unwrap();
                        let res = RecordDao::insert_if_not_exist(Record {
                            content,
                            content_preview: Some(content_preview),
                            data_type: record::DataTypeEnum::IMAGE.into(),
                            ..Default::default()
                        });
                        match res {
                            Ok(_) => {
                                drop(img);
                                need_notify = true;
                            }
                            Err(e) => {
                                error!("insert image record error: {}", e);
                            }
                        }
                        last_img_md5 = img_md5;
                    }
                });

                let limit = AppConfig::latest().lock().record_limit.clone();
                if let Some(l) = limit {
                    let res = RecordDao::delete_record_with_limit(l as usize);
                    if let Ok(success) = res {
                        if success {
                            need_notify = true;
                        }
                    }
                }
                if need_notify {
                    global_handler::GlobalHandler::push_message_to_window(
                        MessageTypeEnum::ChangeClipBoard,
                        "ok",
                    )
                    .unwrap();
                }
                thread::sleep(Duration::milliseconds(Self::WAIT_MILLIS).to_std().unwrap());
            }
        });
    }

    fn handle_text_message() {}

    fn handle_image_message() {}

    fn handle_record_limit() {}
}
