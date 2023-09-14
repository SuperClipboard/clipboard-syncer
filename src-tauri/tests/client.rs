use app::sync_proto::sync_svc_client::SyncSvcClient;
use app::sync_proto::{AddRequest, ListRequest, SyncData};

#[tokio::test]
async fn main() -> anyhow::Result<()> {
    let addr = format!("http://0.0.0.0:{}", "18888");

    let mut cli = SyncSvcClient::connect(addr).await.unwrap();

    cli.add(AddRequest {
        data: Option::from(SyncData {
            md5: "tester33333".to_string(),
            create_time: chrono::Local::now().timestamp() as i32,
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
