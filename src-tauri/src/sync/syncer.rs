use std::collections::HashSet;
use std::sync::OnceLock;

use anyhow::Result;
use backoff::future::retry;
use backoff::ExponentialBackoff;
use local_ip_address::local_ip;
use log::{debug, error, info};
use parking_lot::Mutex;
use tonic::transport::{Channel, Endpoint, Error};

use crate::config::app_config::AppConfig;
use crate::consts::PONG;
use crate::dao::record_dao::RecordDao;
use crate::models::record::Record;
use crate::models::record_cache::RecordCache;
use crate::storage::cache::CacheHandler;
use crate::sync_proto::sync_svc_client::SyncSvcClient;
use crate::sync_proto::{AddRequest, PingRequest, RegisterRequest, RemoveRequest, SyncDataRequest};

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
    pub async fn init_sync_register_list() {
        info!("Start sync data from register list configuration");
        let cfg_register_list;
        {
            cfg_register_list = match AppConfig::latest().read().sync_server_addr_list.clone() {
                None => {
                    return;
                }
                Some(x) => x,
            };
        }
        if cfg_register_list.is_empty() {
            info!("cfg_register_list is empty, exit!");
            return;
        }
        info!(
            "Load cfg_register_list successfully: {:?}, start sync data",
            cfg_register_list
        );

        for addr in cfg_register_list {
            tauri::async_runtime::spawn(async move {
                if let Err(e) = Self::try_register(&addr).await {
                    // Retry failed, remove connection!
                    error!("Try to register addr: {}, error: {}", addr, e);
                    if let Some(sync_server_addr_list) =
                        AppConfig::latest().write().sync_server_addr_list.as_mut()
                    {
                        error!("After multiple retries, {} connect failed", addr);
                        sync_server_addr_list.remove(&addr);
                    }
                };
            });
        }
    }

    pub async fn add_client(addr: String) {
        if Self::check_client_exist(&addr) {
            return;
        }

        let mut s = Syncer::global().lock();
        s.clients.insert(addr.clone());

        let sync_port;
        {
            sync_port = AppConfig::latest().read().sync_port.clone().unwrap();
        }

        tauri::async_runtime::spawn(async move {
            Self::sync_data(&addr, &sync_port).await;
        });
    }

    pub fn check_client_exist(addr: &str) -> bool {
        let s = Self::global().lock();
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
                CacheHandler::global().lock().await.get_copy_data()
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

    async fn sync_data(addr: &str, sync_port: &str) {
        // Step 1: Get client
        let mut client = match Self::get_client(addr).await {
            Ok(c) => c,
            Err(e) => {
                error!("ConnectionRefused: sync data from: {} err: {:#?}", addr, e);
                return;
            }
        };

        // Step 2: Register addr to another server
        let my_local_ip = local_ip().unwrap();
        let data = match client
            .register(RegisterRequest {
                connect_addr: format!("{}:{}", my_local_ip, sync_port),
            })
            .await
        {
            Ok(resp) => resp.into_inner().data,
            Err(e) => {
                error!("Call register err: {:#?}, local ip: {}", e, my_local_ip);
                return;
            }
        };

        // Step 3: Merge diff
        // Calculate diff
        let mut cache = CacheHandler::global().lock().await;
        let diff_md5 = cache.calculate_diff(&data.into_iter().map(|item| item.into()).collect());

        // Update storage
        let diff_records: Vec<Record> = match client
            .sync_data(SyncDataRequest {
                md5_list: diff_md5.iter().map(|record| record.md5.clone()).collect(),
            })
            .await
        {
            Ok(x) => x.into_inner().sync_records,
            Err(e) => {
                error!("Call register err: {:#?}, local ip: {}", e, my_local_ip);
                return;
            }
        }
        .into_values()
        .map(|v| v.into())
        .collect();
        RecordDao::batch_replace_record(diff_records).unwrap();

        // Update cache
        cache.merge_data(&diff_md5);
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

    async fn try_register(client_addr: &str) -> Result<()> {
        retry(ExponentialBackoff::default(), || async move {
            let sync_port;
            {
                sync_port = AppConfig::latest().read().sync_port.clone().unwrap();
            }
            info!(
                "Try register and sync data from: {}:{}",
                client_addr, sync_port
            );
            Syncer::sync_data(client_addr, &sync_port).await;
            Ok(())
        })
        .await
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
