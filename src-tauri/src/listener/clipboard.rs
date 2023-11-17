use arboard::Clipboard;
use chrono::Duration;
use log::{debug, error, info};

use crate::config::app_config::AppConfig;
use crate::dao::record_dao::RecordDao;
use crate::handler::global_handler::GlobalHandler;
use crate::handler::model::MessageTypeEnum;
use crate::models::image_data::ImageData;
use crate::models::record;
use crate::models::record::Record;
use crate::utils::ip::local_ip;
use crate::utils::{image, json, string};

pub struct ClipboardListener;

impl ClipboardListener {
    // Check clipboard content in each 1 second
    const WAIT_MILLIS: i64 = 1000;

    const TEXT_PREVIEW_LEN: usize = 48;

    pub fn listen() {
        tauri::async_runtime::spawn(async {
            let mut last_md5 = String::new();
            let mut clipboard = Clipboard::new().unwrap();
            info!("start clipboard listener");

            loop {
                let mut need_notify = false;
                if let Ok(text) = clipboard.get_text() {
                    Self::handle_text_message(text, &mut last_md5, &mut need_notify).await;
                }
                if let Ok(img) = clipboard.get_image() {
                    Self::handle_image_message(img, &mut last_md5, &mut need_notify).await;
                }

                need_notify = Self::handle_record_limit().await || need_notify;
                if need_notify {
                    GlobalHandler::push_message_to_window(
                        MessageTypeEnum::ChangeClipboardBackend,
                        "Clipboard records changed",
                    )
                    .unwrap();
                }
                tokio::time::sleep(Duration::milliseconds(Self::WAIT_MILLIS).to_std().unwrap())
                    .await;
            }
        });
    }

    async fn handle_text_message(
        text: String,
        last_content_md5: &mut String,
        need_notify: &mut bool,
    ) {
        let content = text.clone();
        let md5 = string::md5(&content);
        if !content.is_empty() && md5.ne(last_content_md5) {
            // Has new clip contents
            let content_preview = if content.len() > Self::TEXT_PREVIEW_LEN {
                Some(
                    content
                        .trim()
                        .chars()
                        .take(Self::TEXT_PREVIEW_LEN)
                        .collect::<String>()
                        + "...",
                )
            } else {
                Some(content.to_string())
            };

            let data = Record {
                content,
                content_preview,
                data_type: record::DataTypeEnum::TEXT.into(),
                latest_addr: local_ip().to_string(),
                ..Default::default()
            };
            debug!("handle_text_message data: {:?}", data);
            let res = RecordDao::insert_if_not_exist(data).await;

            match res {
                Ok(_) => {
                    *need_notify = true;
                }
                Err(e) => {
                    error!("insert record error: {}", e);
                }
            }
            *last_content_md5 = md5;
        }
    }

    async fn handle_image_message(
        img: arboard::ImageData<'_>,
        last_img_md5: &mut String,
        need_notify: &mut bool,
    ) {
        let img_md5 = string::md5_by_bytes(&img.bytes);
        if img_md5.ne(last_img_md5) {
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
            let data = Record {
                content,
                content_preview: Some(content_preview),
                data_type: record::DataTypeEnum::IMAGE.into(),
                latest_addr: local_ip().to_string(),
                ..Default::default()
            };
            let res = RecordDao::insert_if_not_exist(data).await;
            match res {
                Ok(_) => {
                    drop(img);
                    *need_notify = true;
                }
                Err(e) => {
                    error!("insert image record error: {}", e);
                }
            }
            *last_img_md5 = img_md5;
        }
    }

    async fn handle_record_limit() -> bool {
        let limit = AppConfig::latest().read().store_limit.unwrap();
        let res = RecordDao::delete_record_with_limit(limit as usize).await;
        match res {
            Ok(res) => res,
            Err(e) => {
                error!("delete_record_with_limit err: {:?}", e);
                false
            }
        }
    }
}
