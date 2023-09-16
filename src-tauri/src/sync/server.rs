use std::sync::OnceLock;

use anyhow::{bail, Result};
use log::{error, info};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::Mutex;
use tonic::transport::Server;

use crate::sync::service::SyncService;
use crate::sync_proto::sync_svc_server::SyncSvcServer;

pub struct ServerHandler {
    shutdown_signer: (Sender<()>, Receiver<()>),
}

impl ServerHandler {
    // init global shutdown signal
    pub fn global() -> &'static Mutex<ServerHandler> {
        static SERVER_HANDLER: OnceLock<Mutex<ServerHandler>> = OnceLock::new();

        SERVER_HANDLER.get_or_init(|| {
            Mutex::new(ServerHandler {
                shutdown_signer: tokio::sync::broadcast::channel::<()>(2),
            })
        })
    }

    pub async fn start(&mut self, port: &str) -> Result<()> {
        let addr = format!("0.0.0.0:{}", port);
        info!("Listening on port {}", addr);

        Server::builder()
            .add_service(SyncSvcServer::new(SyncService))
            .serve_with_shutdown(addr.parse()?, async move {
                match self.shutdown_signer.1.blocking_recv() {
                    Ok(_) => {
                        info!("Server is about to shutdown!");
                    }
                    Err(e) => {
                        error!("Failed to shutdown server: {:?}", e);
                    }
                };
            })
            .await?;

        Ok(())
    }

    pub async fn shutdown(&self) -> Result<()> {
        // Old server is running, shutdown now
        match self.shutdown_signer.0.send(()) {
            Ok(_) => {
                info!("Send shutdown signal to server successfully!");
                Ok(())
            }
            Err(e) => {
                error!("Error sending shutdown signal to server: {}", e);
                bail!("Error sending shutdown signal to server: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::app_config::AppConfig;
    use crate::sync::server::ServerHandler;

    #[tokio::test]
    async fn test_restart() {
        let sync_port;
        {
            sync_port = AppConfig::latest().read().sync_port.clone().unwrap();
        }

        let sync_port1 = sync_port.clone();
        tokio::spawn(async move {
            ServerHandler::global()
                .lock()
                .await
                .start(&sync_port1)
                .await
                .unwrap();
            println!("Server started!");
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
        println!(
            "shutdown result: {:?}",
            ServerHandler::global().lock().await.shutdown().await
        );
        println!("Server shutdown!");

        tokio::spawn(async move {
            ServerHandler::global()
                .lock()
                .await
                .start(&sync_port)
                .await
                .unwrap();
            println!("Server restarted!");
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
        println!(
            "re-shutdown result: {:?}",
            ServerHandler::global().lock().await.shutdown().await
        );
        println!("Server re-shutdown!");
    }
}
