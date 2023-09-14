use crate::consts::SYNC_PORT;
use crate::sync::service::SyncService;
use crate::sync_proto::sync_svc_server::SyncSvcServer;
use log::info;
use tonic::transport::Server;

pub async fn serve() -> anyhow::Result<()> {
    let addr = format!("0.0.0.0:{}", SYNC_PORT);
    info!("Listening on port {}", addr);

    Server::builder()
        .add_service(SyncSvcServer::new(SyncService))
        .serve(addr.parse()?)
        .await?;

    Ok(())
}
