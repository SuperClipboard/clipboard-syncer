use anyhow::{bail, Result};
use aquadoggo::{Configuration, NetworkConfiguration, Node};
use log::{debug, info};
use p2panda_rs::entry::decode::decode_entry;
use p2panda_rs::entry::traits::AsEntry;

use crate::config::app_config::AppConfig;
use crate::consts::SQLITE_FILE;
use crate::p2panda::graphql::GraphQLHandler;
use crate::p2panda::key_pair::get_key_pair;
use crate::p2panda::models::LockFile;
use crate::utils::dir::app_data_dir;

pub struct NodeServer;

impl NodeServer {
    /// Start the p2panda node server
    pub async fn start() -> Result<Node> {
        // Step 1: Load the configuration
        let graphql_port;
        let sync_port;
        let database_uri;
        {
            let config = AppConfig::latest();
            graphql_port = config.read().graphql_port.clone().unwrap().parse::<u16>()?;
            sync_port = config.read().sync_port.clone().unwrap().parse::<u16>()?;
            database_uri = format!(
                "sqlite://{}",
                app_data_dir()?.join(SQLITE_FILE).to_str().unwrap()
            );
        }

        let config = Configuration {
            database_url: database_uri,
            http_port: graphql_port,
            network: NetworkConfiguration {
                quic_port: sync_port,
                ..Default::default()
            },
            ..Default::default()
        };
        let key_pair = get_key_pair()?;

        // Step 2: Start node server
        let node = Node::start(key_pair, config).await;
        info!(
            "Init node server success, graphql port: {}, node port: {}",
            graphql_port, sync_port
        );

        // Step 3: Do schema migration
        NodeServer::migration().await?;
        info!("Node server migration task done");

        Ok(node)
    }

    async fn migration() -> Result<()> {
        let mut cli = GraphQLHandler::global().lock().await;

        let data = include_str!("../../schema/schema.lock");
        let lock_file: LockFile = toml::from_str(data)?;

        // Iterate over all commits which are required to migrate to the latest
        // version. This loop automatically checks if the commit already took place
        // and ignores them if so
        let mut migration_task_cnt = 0;
        for commit in lock_file.commits.unwrap() {
            // Decode entry from commit to retrieve public key, sequence number and log id from it
            let entry = decode_entry(&commit.entry)?;
            let pub_key = entry.public_key();
            let log_id = entry.log_id();
            let seq_num = entry.seq_num();

            // Check if node already knows about this entry
            let next_args_res = cli
                .next_args(*pub_key, Some(commit.entry_hash.to_string()))
                .await;
            if next_args_res.is_ok() {
                let next_args_res = next_args_res.unwrap();
                // Has previous record already
                if next_args_res.is_some() {
                    let next_args_res = next_args_res.unwrap();
                    if log_id != &next_args_res.log_id {
                        bail!("Critical log id mismatch during migration")
                    }
                    // Entry already exists, we can ignore this commit
                    if seq_num < &next_args_res.seq_num {
                        continue;
                    }
                }
            }

            // Publish commit to node, this will materialize the (updated) schema on
            // the node and give us a new GraphQL API
            let pub_res = cli.publish(commit.entry, commit.operation).await?;
            debug!("publish res: {:?}", pub_res);
            migration_task_cnt += 1;
        }

        if migration_task_cnt <= 0 {
            info!("No migration tasks needed!");
        } else {
            info!("Done {} migration tasks", migration_task_cnt);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::p2panda::node::NodeServer;

    #[tokio::test]
    async fn test_build() {
        let _node = NodeServer::start().await.unwrap();
        // _node.on_exit().await;
    }
}
