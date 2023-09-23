pub mod command;
pub mod config;
pub mod consts;
pub mod dao;
pub mod handler;
pub mod listener;
pub mod logger;
pub mod models;
pub mod schema;
pub mod storage;
pub mod sync;
pub mod tray;
pub mod utils;

pub mod sync_proto {
    include!("./proto-gen/sync.rs");
}
