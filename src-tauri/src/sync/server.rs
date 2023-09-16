use std::sync::OnceLock;

use anyhow::Result;
use log::info;
use tokio::sync::Mutex;
use tonic::transport::Server;

use crate::sync::service::SyncService;
use crate::sync_proto::sync_svc_server::SyncSvcServer;

pub struct ServerHandler;

impl ServerHandler {
    // init global shutdown signal
    pub fn global() -> &'static Mutex<ServerHandler> {
        static SERVER_HANDLER: OnceLock<Mutex<ServerHandler>> = OnceLock::new();

        SERVER_HANDLER.get_or_init(|| Mutex::new(ServerHandler {}))
    }

    pub async fn start(&mut self, port: &str) -> Result<()> {
        let addr = format!("0.0.0.0:{}", port);
        info!("Listening on port {}", addr);

        Server::builder()
            .add_service(SyncSvcServer::new(SyncService))
            .serve(addr.parse()?)
            .await?;

        Ok(())
    }
}
