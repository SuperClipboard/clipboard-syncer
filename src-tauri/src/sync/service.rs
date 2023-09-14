use std::string::ToString;

use log::debug;
use tonic::{Request, Response, Status};

use crate::consts::PONG;
use crate::storage::cache::CacheHandler;
use crate::sync::syncer::{SyncOptEnum, Syncer};
use crate::sync_proto::sync_svc_server::{SyncSvc};
use crate::sync_proto::{
    AddRequest, AddResponse, ListRequest, ListResponse, PingRequest, PingResponse, RegisterRequest,
    RegisterResponse, RemoveRequest, RemoveResponse,
};

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
        let store = CacheHandler::global().lock();
        Ok(Response::new(ListResponse {
            data: store.get_copy_data().into_iter().collect(),
        }))
    }

    async fn add(&self, req: Request<AddRequest>) -> Result<Response<AddResponse>, Status> {
        let key = req.into_inner().key;

        let mut store = CacheHandler::global().lock();
        if store.contains(&key) {
            debug!("Already contains data: {}, skip...", key);
            return Ok(Response::new(AddResponse {}));
        }

        debug!("add store: {}, success", key);
        store.add(key.clone());
        Syncer::sync_opt(SyncOptEnum::Add, key);
        Ok(Response::new(AddResponse {}))
    }

    async fn remove(
        &self,
        req: Request<RemoveRequest>,
    ) -> Result<Response<RemoveResponse>, Status> {
        let k = req.into_inner().key;

        let mut store = CacheHandler::global().lock();
        if !store.contains(&k) {
            debug!("Not contains data: {}, skip...", k);
            return Ok(Response::new(RemoveResponse {}));
        }

        debug!("remove store: {}, success", k);
        store.remove(&k);
        Syncer::sync_opt(SyncOptEnum::Remove, k);
        Ok(Response::new(RemoveResponse {}))
    }

    async fn register(
        &self,
        req: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let connect_addr = req.into_inner().connect_addr;
        Syncer::add_client(connect_addr).await;
        let store = CacheHandler::global().lock();
        Ok(Response::new(RegisterResponse {
            data: store.get_copy_data().into_iter().collect(),
        }))
    }
}
