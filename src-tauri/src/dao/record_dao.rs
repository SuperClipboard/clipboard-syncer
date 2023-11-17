use std::str::FromStr;

use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};
use local_ip_address::local_ip;
use log::{debug, error, info};
use p2panda_rs::document::DocumentViewId;
use p2panda_rs::operation::plain::PlainOperation;
use p2panda_rs::operation::{OperationAction, OperationBuilder, OperationId, OperationValue};
use p2panda_rs::schema::SchemaId;

use crate::config::app_config::AppConfig;
use crate::graphql::record_by_pages::OrderDirection;
use crate::graphql::{
    record_by_md5_query, record_by_pages, record_counts, GraphRecordDocuments, GraphRecordOrderBy,
    GraphRecordPageDocuments, RecordByMd5Query, RecordByPages, RecordCounts,
};
use crate::handler::global_handler::GlobalHandler;
use crate::handler::model::MessageTypeEnum;
use crate::models::record;
use crate::models::record::Record;
use crate::p2panda::graphql::GraphQLHandler;
use crate::utils::string;

pub struct RecordDao;

impl RecordDao {
    pub async fn insert_if_not_exist(mut r: Record) -> Result<()> {
        let now = chrono::Local::now().timestamp();
        let md5_str = string::md5(r.content.as_str());
        r.md5 = md5_str.clone();
        r.create_time = now;

        let res = RecordDao::find_record_by_md5(md5_str).await?;
        match res.len() {
            // no record
            0 => {
                Self::create_record(r).await?;
                debug!("insert new record successfully with len 0");
            }
            // find record
            _ => {
                Self::update_record_with_fields(
                    &OperationId::from_str(&res[0].meta.as_ref().unwrap().view_id)?.into(),
                    &[
                        ("create_time", OperationValue::Integer(now)),
                        (
                            "latest_addr",
                            OperationValue::String(local_ip().unwrap().to_string()),
                        ),
                    ],
                )
                .await?;
                debug!("update record successfully: {:?}", r);
            }
        };
        Ok(())
    }

    pub async fn create_record(record: Record) -> Result<String> {
        let opt = OperationBuilder::new(&SchemaId::new(record::SCHEMA_ID).unwrap())
            .action(OperationAction::Create)
            .fields(&[
                ("content", OperationValue::String(record.content)),
                (
                    "content_preview",
                    OperationValue::String(record.content_preview.unwrap_or(String::new())),
                ),
                ("data_type", OperationValue::String(record.data_type)),
                ("md5", OperationValue::String(record.md5)),
                ("create_time", OperationValue::Integer(record.create_time)),
                ("is_favorite", OperationValue::Integer(record.is_favorite)),
                ("tags", OperationValue::String(record.tags)),
                ("latest_addr", OperationValue::String(record.latest_addr)),
                ("is_deleted", OperationValue::Integer(record.is_deleted)),
            ])
            .build()?;

        let mut handler = GraphQLHandler::global().lock().await;
        let res = handler.send_to_node(PlainOperation::from(&opt)).await?;
        info!("create record success, opt id: {}", res);
        Ok(res)
    }

    pub async fn find_record_by_md5(md5: String) -> Result<Vec<GraphRecordDocuments>> {
        let handler = &mut GraphQLHandler::global().lock().await;

        let request_body = RecordByMd5Query::build_query(record_by_md5_query::Variables { md5 });

        let res = handler
            .cli
            .post(handler.endpoint())
            .json(&request_body)
            .send()
            .await?;
        let response_body: Response<record_by_md5_query::ResponseData> = res.json().await?;

        match response_body.data {
            None => Ok(vec![]),
            Some(res) => Ok(res
                .all_record_002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099f
                .documents),
        }
    }

    pub async fn update_record_with_fields(
        view_id: &DocumentViewId,
        fields: &[(impl ToString, OperationValue)],
    ) -> Result<String> {
        let opt = OperationBuilder::new(&SchemaId::new(record::SCHEMA_ID).unwrap())
            .action(OperationAction::Update)
            .previous(view_id)
            .fields(fields)
            .build()?;

        let handler = &mut GraphQLHandler::global().lock().await;
        let res = handler.send_to_node(PlainOperation::from(&opt)).await?;
        info!("update record success, opt id: {}", res);
        Ok(res)
    }

    pub async fn delete_record(view_id: &DocumentViewId) -> Result<String> {
        let opt = OperationBuilder::new(&SchemaId::new(record::SCHEMA_ID).unwrap())
            .action(OperationAction::Delete)
            .previous(view_id)
            .build()?;

        let handler = &mut GraphQLHandler::global().lock().await;
        let res = handler.send_to_node(PlainOperation::from(&opt)).await?;
        info!("delete record success, opt id: {}", res);
        Ok(res)
    }

    // Delete record with over limit
    pub async fn delete_record_with_limit(limit: usize) -> Result<bool> {
        // 先查询count，如果count - limit > RECORD_LIMIT_THRESHOLD 才删除超出limit部分记录，防止频繁操作
        let cnt = Self::count_records().await? as usize;

        // Not reach the threshold
        let record_limit_threshold;
        {
            record_limit_threshold = AppConfig::latest().read().record_limit_threshold.unwrap();
        }
        if cnt < record_limit_threshold + limit {
            return Ok(false);
        }

        let actual_remove_cnt = (cnt - limit) as i64;
        info!(
            "[delete_record_with_limit] {} records needed to remove",
            actual_remove_cnt
        );

        let need_delete_records = Self::record_by_pages(
            Some(actual_remove_cnt),
            None,
            Some(vec![0]),
            Some(GraphRecordOrderBy::create_time),
            Some(OrderDirection::ASC),
        )
        .await?;

        // Delete records
        let need_delete_records_len = need_delete_records.len();
        Self::batch_delete_record(need_delete_records).await?;

        if let Err(e) = GlobalHandler::push_message_to_window(
            MessageTypeEnum::DeleteClipboardRecordBackend,
            format!("batch delete {} records success", need_delete_records_len),
        ) {
            error!("send DeleteClipboardRecordBackend message err: {:?}", e)
        };

        Ok(true)
    }

    async fn count_records() -> Result<i64> {
        let handler = &mut GraphQLHandler::global().lock().await;
        let request_body = RecordCounts::build_query(record_counts::Variables {
            favorite_filter: Some(vec![0]),
        });
        let res = handler
            .cli
            .post(handler.endpoint())
            .json(&request_body)
            .send()
            .await?;
        let response_body: Response<record_counts::ResponseData> = res.json().await?;

        match response_body.data {
            None => Ok(0),
            Some(res) => Ok(res
                .all_record_002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099f
                .total_count),
        }
    }

    async fn record_by_pages(
        limit: Option<i64>,
        start_cursor: Option<String>,
        favorite_filter: Option<Vec<i64>>,
        order_by: Option<GraphRecordOrderBy>,
        order_dir: Option<OrderDirection>,
    ) -> Result<Vec<GraphRecordPageDocuments>> {
        let handler = &mut GraphQLHandler::global().lock().await;
        let request_body = RecordByPages::build_query(record_by_pages::Variables {
            order_by,
            order_dir,
            limit,
            start_cursor,
            favorite_filter,
        });

        let res = handler
            .cli
            .post(handler.endpoint())
            .json(&request_body)
            .send()
            .await?;
        let response_body: Response<record_by_pages::ResponseData> = res.json().await?;

        match response_body.data {
            None => Ok(vec![]),
            Some(res) => Ok(res
                .all_record_002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099f
                .documents),
        }
    }

    async fn batch_delete_record(need_delete_records: Vec<GraphRecordPageDocuments>) -> Result<()> {
        let handler = &mut GraphQLHandler::global().lock().await;

        for record in need_delete_records {
            let opt = OperationBuilder::new(&SchemaId::new(record::SCHEMA_ID).unwrap())
                .action(OperationAction::Delete)
                .previous(
                    &OperationId::from_str(record.meta.as_ref().unwrap().document_id.as_str())?
                        .into(),
                )
                .build()?;

            match handler.send_to_node(PlainOperation::from(&opt)).await {
                Ok(res) => {
                    info!("delete record success, opt id: {}", res);
                }
                Err(err) => {
                    error!("delete record error: {}", err);
                }
            };
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::dao::record_dao::RecordDao;
    use crate::models::record::Record;
    use crate::p2panda::node::NodeServer;

    #[tokio::test]
    #[ignore]
    async fn test_create_record() {
        NodeServer::start().await.unwrap();

        let res = RecordDao::create_record(Record {
            content: "test".to_string(),
            content_preview: None,
            data_type: "".to_string(),
            md5: "123".to_string(),
            create_time: 1699845357,
            tags: "".to_string(),
            latest_addr: "".to_string(),
            ..Default::default()
        })
        .await
        .unwrap();

        println!("test_create_record: {}", res);
    }
}
