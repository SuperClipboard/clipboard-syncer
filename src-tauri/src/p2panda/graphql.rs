use std::sync::OnceLock;

use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};
use log::{debug, info, warn};
use p2panda_rs::entry::encode::sign_and_encode_entry;
use p2panda_rs::entry::traits::AsEncodedEntry;
use p2panda_rs::identity::PublicKey;
use p2panda_rs::operation::encode::encode_plain_operation;
use p2panda_rs::operation::plain::PlainOperation;
use p2panda_rs::operation::traits::Actionable;
use reqwest::Client;
use tokio::sync::Mutex;

use crate::config::app_config::AppConfig;
use crate::graphql::next_args_query::NextArgsQueryNextArgs;
use crate::graphql::publish_mut::PublishMutPublish;
use crate::graphql::{
    next_args_query, publish_mut, EncodedEntry, EncodedOperation, NextArgsQuery, PublishMut,
};
use crate::p2panda::key_pair::get_key_pair;

#[derive(Debug)]
pub struct GraphQLHandler {
    endpoint: String,
    pub cli: Client,
}

impl GraphQLHandler {
    // init global
    pub fn global() -> &'static Mutex<GraphQLHandler> {
        static CLIENT: OnceLock<Mutex<GraphQLHandler>> = OnceLock::new();

        CLIENT.get_or_init(|| Mutex::new(GraphQLHandler::new()))
    }

    pub async fn next_args(
        &mut self,
        public_key: PublicKey,
        view_id: Option<String>,
    ) -> Result<Option<NextArgsQueryNextArgs>> {
        let request_body = NextArgsQuery::build_query(next_args_query::Variables {
            pk: public_key,
            vid: view_id,
        });

        let res = self
            .cli
            .post(&self.endpoint)
            .json(&request_body)
            .send()
            .await?;
        let response_body: Response<next_args_query::ResponseData> = res.json().await?;

        debug!("next_args response: {:?}", response_body);

        match response_body.data {
            None => Ok(None),
            Some(res) => Ok(res.next_args),
        }
    }

    pub async fn publish(
        &mut self,
        encoded_entry: EncodedEntry,
        encoded_operation: EncodedOperation,
    ) -> Result<Option<PublishMutPublish>> {
        let request_body = PublishMut::build_query(publish_mut::Variables {
            entry: encoded_entry,
            operation: encoded_operation,
        });
        let res = self
            .cli
            .post(&self.endpoint)
            .json(&request_body)
            .send()
            .await?;
        let response_body: Response<publish_mut::ResponseData> = res.json().await?;

        debug!("publish response: {:?}", response_body);

        match response_body.data {
            None => Ok(None),
            Some(res) => Ok(Some(res.publish)),
        }
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub async fn send_to_node(&mut self, operation: PlainOperation) -> Result<String> {
        // 1. Load public key from key_pair
        let key_pair = get_key_pair()?;

        // 2. Send `nextArgs` GraphQL query to get the arguments from the node to create the next entry
        let next_args = self
            .next_args(
                key_pair.public_key(),
                // Set `viewId` when `previous` is given in operation
                operation.previous().map(|id| id.to_string()),
            )
            .await?;

        if next_args.is_none() {
            warn!("Get next arguments failed: {:?}", next_args);
            return Ok(String::new());
        }

        let NextArgsQueryNextArgs {
            log_id,
            seq_num,
            skiplink,
            backlink,
        } = next_args.unwrap();

        // 3. Create p2panda data! Encode operation, sign and encode entry
        let encoded_operation = encode_plain_operation(&operation)?;
        let encoded_entry = sign_and_encode_entry(
            &log_id,
            &seq_num,
            skiplink.as_ref(),
            backlink.as_ref(),
            &encoded_operation,
            &key_pair,
        )?;

        let operation_id = encoded_entry.hash();
        self.publish(encoded_entry, encoded_operation).await?;

        Ok(operation_id.to_string())
    }

    fn new() -> Self {
        let graphql_port;
        {
            graphql_port = AppConfig::latest().read().graphql_port.unwrap();
        }
        let endpoint = format!("http://localhost:{}/graphql", graphql_port);
        let cli = Client::new();

        info!("Init graphql client success endpoint: {}", endpoint);

        Self { cli, endpoint }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::configure::Configure;
    use crate::p2panda::graphql::GraphQLHandler;

    #[test]
    fn test_new_config() {
        let cfg = Configure::new();
        println!("{:?}", cfg);

        let cli = GraphQLHandler::global();
        println!("client: {:?}", cli)
    }
}
