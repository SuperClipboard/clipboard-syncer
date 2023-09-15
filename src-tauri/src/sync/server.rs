use crate::config::app_config::AppConfig;
use crate::sync::service::SyncService;
use crate::sync_proto::sync_svc_server::SyncSvcServer;
use log::info;
use tonic::transport::Server;

pub async fn serve() -> anyhow::Result<()> {
    let sync_port;
    {
        sync_port = AppConfig::latest().lock().sync_port.clone();
    }
    let addr = format!("0.0.0.0:{}", sync_port);
    info!("Listening on port {}", addr);

    Server::builder()
        .add_service(SyncSvcServer::new(SyncService))
        .serve(addr.parse()?)
        .await?;

    Ok(())
}
