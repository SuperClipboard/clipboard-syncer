use app::sync_proto::sync_svc_client::SyncSvcClient;
use app::sync_proto::{AddRequest, ListRequest, SyncRecord};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = format!("http://127.0.0.1:{}", "18888");

    let mut cli = SyncSvcClient::connect(addr).await.unwrap();

    cli.add(AddRequest {
        data: Option::from(SyncRecord {
            md5: "tester".to_string(),
            create_time: chrono::Local::now().timestamp() as i32,
            ..Default::default()
        }),
    })
    .await
    .unwrap();

    println!(
        "list: {:#?}",
        cli.list(ListRequest {}).await.unwrap().into_inner().data
    );

    Ok(())
}
