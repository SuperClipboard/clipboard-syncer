use std::collections::HashSet;
use std::sync::OnceLock;

use anyhow::Result;
use local_ip_address::local_ip;
use log::{debug, error, info};
use parking_lot::Mutex;
use tonic::transport::{Channel, Endpoint, Error};

use crate::consts::{PONG, SYNC_PORT};
use crate::models::record_cache::RecordCache;
use crate::storage::cache::CacheHandler;
use crate::sync_proto::sync_svc_client::SyncSvcClient;
use crate::sync_proto::{AddRequest, PingRequest, RegisterRequest, RemoveRequest};

#[derive(Debug, Default)]
pub struct Syncer {
    clients: HashSet<String>,
}

#[derive(Debug)]
pub enum SyncOptEnum {
    Add,
    Remove,
}

impl Syncer {
    pub async fn add_client(addr: String) {
        if Self::check_client_exist(&addr) {
            return;
        }

        let mut s = Syncer::global().lock();
        s.clients.insert(addr.clone());

        tauri::async_runtime::spawn(async move {
            Self::sync_data(&addr).await;
        });
    }

    pub fn check_client_exist(addr: &str) -> bool {
        let s = Syncer::global().lock();
        s.clients.contains(addr)
    }

    pub fn sync_opt(opt: SyncOptEnum, data: RecordCache) {
        let client_list;
        {
            client_list = Self::global().lock().clients.clone();
        }
        let mut disconnected_addr = vec![];

        debug!(
            "Start sync opt: {:?}, current client list: {:#?}",
            opt, client_list
        );

        tauri::async_runtime::spawn(async move {
            for client_addr in client_list.iter() {
                // Step 1: Get client
                let mut rpc_cli = match Self::get_client(client_addr).await {
                    Ok(cli) => cli,
                    Err(e) => {
                        error!("Get client for address: {} err: {}", client_addr, e);
                        disconnected_addr.push(client_addr.clone());
                        continue;
                    }
                };
                // Step 2: Check health
                match Self::check_health(&mut rpc_cli).await {
                    Ok(is_health) => {
                        if !is_health {
                            error!("Checked unhealthy for address: {}", client_addr);
                            disconnected_addr.push(client_addr.clone());
                            continue;
                        }
                    }
                    Err(e) => {
                        error!(
                            "Check health failed for address: {} err: {}",
                            client_addr, e
                        );
                        disconnected_addr.push(client_addr.clone());
                        continue;
                    }
                }
                // Step 3: Opt
                match opt {
                    SyncOptEnum::Add => {
                        Self::sync_add(&mut rpc_cli, data.clone(), client_addr).await;
                    }
                    SyncOptEnum::Remove => {
                        Self::sync_remove(&mut rpc_cli, data.clone(), client_addr).await;
                    }
                };
            }
            debug!("Sync Opt: {:?} success, data: {:?}", opt, data);
            debug!(
                "Current data: {:#?}",
                CacheHandler::global().lock().get_copy_data()
            )
        });
    }

    fn global() -> &'static Mutex<Syncer> {
        static SYNCER: OnceLock<Mutex<Syncer>> = OnceLock::new();

        SYNCER.get_or_init(|| {
            let d = Mutex::new(Syncer::new());
            info!("init syncer success!");
            d
        })
    }

    fn new() -> Self {
        Self {
            clients: HashSet::new(),
        }
    }

    async fn check_health(client: &mut SyncSvcClient<Channel>) -> Result<bool> {
        let resp = client.ping(PingRequest {}).await?;
        Ok(resp.into_inner().msg.eq(PONG))
    }

    async fn sync_data(addr: &str) {
        let mut client = match Self::get_client(addr).await {
            Ok(c) => c,
            Err(e) => {
                error!("ConnectionRefused: sync data from: {} err: {:#?}", addr, e);
                return;
            }
        };

        let my_local_ip = local_ip().unwrap();
        let data = match client
            .register(RegisterRequest {
                connect_addr: format!("{}:{}", my_local_ip, SYNC_PORT),
            })
            .await
        {
            Ok(resp) => resp.into_inner().data,
            Err(e) => {
                error!("Call register err: {:#?}, local ip: {}", e, my_local_ip);
                return;
            }
        };

        let mut store = CacheHandler::global().lock();
        store.merge_data(&data.into_iter().map(|item| item.into()).collect());
    }

    async fn get_client(addr: &str) -> Result<SyncSvcClient<Channel>, Error> {
        let addr = Endpoint::from_shared(format!("http://{}", addr))?;
        SyncSvcClient::connect(addr).await
    }

    async fn sync_add(client: &mut SyncSvcClient<Channel>, data: RecordCache, addr: &str) {
        match client
            .add(AddRequest {
                data: Some(data.clone().into()),
            })
            .await
        {
            Err(e) => {
                error!("Sync[Add] data: {:?} for addr: {} error: {}", data, addr, e);
            }
            _ => {
                debug!("Sync[Add] data: {:?} for addr: {} success", data, addr)
            }
        };
    }

    async fn sync_remove(client: &mut SyncSvcClient<Channel>, data: RecordCache, addr: &str) {
        match client
            .remove(RemoveRequest {
                data: Some(data.clone().into()),
            })
            .await
        {
            Err(e) => {
                error!(
                    "Sync[Remove] data: {:?} for addr: {} error: {}",
                    data, addr, e
                );
            }
            _ => {
                debug!("Sync[Remove] data: {:?} for addr: {} success", data, addr)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use local_ip_address::local_ip;

    #[test]
    fn test_ip() {
        let my_local_ip = local_ip().unwrap();
        println!("This is my local IP address: {:?}", my_local_ip);
    }
}
