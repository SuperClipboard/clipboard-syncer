use std::collections::HashMap;
use std::string::ToString;

use local_ip_address::local_ip;
use log::{debug, error, warn};
use tonic::{Request, Response, Status};

use crate::consts::{LOCALHOST, PONG, UNKNOWN_HOST};
use crate::dao::record_dao::RecordDao;
use crate::storage::cache::CacheHandler;
use crate::sync::syncer::{SyncOptEnum, Syncer};
use crate::sync_proto::sync_svc_server::SyncSvc;
use crate::sync_proto::{
    AddRequest, AddResponse, ListRequest, ListResponse, PingRequest, PingResponse, RegisterRequest,
    RegisterResponse, RemoveRequest, RemoveResponse, SyncDataRequest, SyncDataResponse, SyncRecord,
};
use crate::utils::ip;

#[derive(Default)]
pub struct SyncService;

#[tonic::async_trait]
impl SyncSvc for SyncService {
    async fn ping(&self, _req: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        Ok(Response::new(PingResponse {
            msg: PONG.to_string(),
        }))
    }

    async fn list(&self, _req: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        let store = CacheHandler::global().blocking_lock();
        Ok(Response::new(ListResponse {
            data: store
                .get_copy_data()
                .into_iter()
                .map(|item| item.into())
                .collect(),
        }))
    }

    async fn add(&self, req: Request<AddRequest>) -> Result<Response<AddResponse>, Status> {
        let data = match req.into_inner().data {
            None => {
                warn!("Add request is empty!");
                return Err(Status::invalid_argument("Request is empty"));
            }
            Some(data) => data,
        };

        let mut store = CacheHandler::global().blocking_lock();
        if store.contains(&data.clone().into()) {
            debug!("Already contains data: {:?}, skip...", data);
            return Ok(Response::new(AddResponse {}));
        }

        debug!("add store: {:?}, success", data);
        store.add(data.clone().into());
        Syncer::sync_opt(SyncOptEnum::Add, data.into());
        Ok(Response::new(AddResponse {}))
    }

    async fn remove(
        &self,
        req: Request<RemoveRequest>,
    ) -> Result<Response<RemoveResponse>, Status> {
        let data = match req.into_inner().data {
            None => {
                warn!("Remove request is empty!");
                return Err(Status::invalid_argument("Request is empty"));
            }
            Some(data) => data,
        };

        let mut store = CacheHandler::global().blocking_lock();
        if !store.contains(&data.clone().into()) {
            debug!("Not contains data: {:?}, skip...", data);
            return Ok(Response::new(RemoveResponse {}));
        }

        debug!("remove store: {:?}, success", data);
        store.remove(&data.clone().into());
        Syncer::sync_opt(SyncOptEnum::Remove, data.into());
        Ok(Response::new(RemoveResponse {}))
    }

    async fn register(
        &self,
        req: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let connect_addr = req.into_inner().connect_addr;
        if !check_address(&connect_addr) {
            let err_msg = format!("Invalid address: {:?}", connect_addr);
            warn!("{}", err_msg);
            return Err(Status::invalid_argument(err_msg));
        }

        Syncer::add_client(connect_addr).await;
        let store = CacheHandler::global().blocking_lock();
        Ok(Response::new(RegisterResponse {
            data: store
                .get_copy_data()
                .into_iter()
                .map(|item| item.into())
                .collect(),
        }))
    }

    async fn sync_data(
        &self,
        req: Request<SyncDataRequest>,
    ) -> Result<Response<SyncDataResponse>, Status> {
        let md5_list = req.into_inner().md5_list;

        if md5_list.is_empty() {
            return Ok(Response::new(SyncDataResponse {
                sync_records: HashMap::new(),
            }));
        }

        let mut records = match RecordDao::find_records_in_md5_list(&md5_list) {
            Ok(x) => x,
            Err(e) => {
                error!("Call find_records_in_md5_list err: {:?}", e);
                return Err(Status::internal(format!("find records failed: {}", e)));
            }
        };

        // Update record address
        records.iter_mut().for_each(|record| {
            if record.latest_addr.eq(LOCALHOST) {
                record.latest_addr =
                    local_ip().map_or(UNKNOWN_HOST.to_string(), |ip| ip.to_string());
            }
        });

        return Ok(Response::new(SyncDataResponse {
            sync_records: records
                .into_iter()
                .map(|record| (record.md5.clone(), record.into()))
                .collect::<HashMap<String, SyncRecord>>(),
        }));
    }
}

fn check_address(addr: &str) -> bool {
    ip::check_addr(addr) && addr.ne(LOCALHOST)
}
