use std::fmt::Display;
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

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum OperationAction {
    Create = 0,
    Update = 1,
    Delete = 2,
}

impl Display for OperationAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as usize)
    }
}

pub fn field(a: &str, b: &str) -> StringTuple {
    (a.to_string(), b.to_string())
}

pub type StringTuple = (String, String);

#[derive(Debug)]
pub struct GraphQLHandler {
    version: usize,
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

    /// Creates an instance following the shape of the schema with the respective schema_id
    pub async fn create_instance(
        &mut self,
        schema_id: &str,
        fields: &mut [StringTuple],
    ) -> Result<String> {
        sort_fields(fields);
        let payload_content: Vec<String> = fields_to_json_fields(fields);

        // [1, 0, "chat_0020cae3b...", {"msg": "...", "username": "..." } ]

        let json = format!(
            r#"[{}, {}, "{}", {{ {} }} ]"#,
            self.version,
            OperationAction::Create,
            schema_id,
            payload_content.join(", ")
        );

        self.send_to_node(&json).await
    }

    /// Updates partially or completely an instance with the respective view_id
    pub async fn update_instance(
        &mut self,
        schema_id: &str,
        view_id: &str,
        fields: &mut [StringTuple],
    ) -> Result<String> {
        sort_fields(fields);
        let to_update: Vec<String> = fields_to_json_fields(fields);

        //[1, 1, "chat_0020cae3b...", [ "<view_id>" ], { "username": "..." }]

        let json = format!(
            r#"[{}, {}, "{}", [ "{}" ], {{ {} }} ]"#,
            self.version,
            OperationAction::Update,
            schema_id,
            view_id,
            to_update.join(", ")
        );

        self.send_to_node(&json).await
    }

    /// Deletes an instance with the respective view_id
    pub async fn delete_instance(&mut self, schema_id: &str, view_id: &str) -> Result<String> {
        let json = format!(
            r#"[ {},{},"{}",["{}"] ]"#,
            self.version,
            OperationAction::Delete,
            schema_id,
            view_id
        );

        self.send_to_node(&json).await
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    async fn send_to_node(&mut self, json: &str) -> Result<String> {
        // 1. Load public key from key_pair
        let key_pair = get_key_pair()?;

        // 2. Parse operation from JSON string
        let operation: PlainOperation = serde_json::from_str(json)?;

        // 3. Send `nextArgs` GraphQL query to get the arguments from the node to create the next entry
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

        // 4. Create p2panda data! Encode operation, sign and encode entry
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
            graphql_port = AppConfig::latest().read().graphql_port.clone().unwrap();
        }
        let endpoint = format!("http://localhost:{}/graphql", graphql_port);
        let cli = Client::new();

        info!("Init graphql client success endpoint: {}", endpoint);

        Self {
            version: 1,
            cli,
            endpoint,
        }
    }
}

/// Utility function to sort `Vec<StringTuple>` in alphabetical order
/// p2panda requires the fields in alphabetical order
pub fn sort_fields(fields: &mut [StringTuple]) {
    fields.sort_by(|a, b| a.0.cmp(&b.0))
}

/// Utility function to map a `Vec<StringTuple>` to `Vec<String>`
/// The resulting string has the shape: `"a": "b"` or `"a": b` if b is a number or boolean
pub fn fields_to_json_fields(fields: &[StringTuple]) -> Vec<String> {
    fields.iter().map(field_to_json).collect()
}

/// Transforms a StringTuple (name and value) to a json field
/// ### Example:
/// input: `(PI, 3.1416)` output: `"PI": 3.1416`
pub fn field_to_json((name, value): &StringTuple) -> String {
    let value = (*value).to_string();

    if value == "true" || value == "false" {
        return format!(r#""{}": {}"#, name, value);
    }

    // For relation_list, pinned_relation and pinned_relation_list
    if value.starts_with('[') && value.ends_with(']') {
        return format!(r#""{}": {}"#, name, value);
    }

    if let Ok(x) = value.parse::<f64>() {
        return if value.contains('.') {
            format!(r#""{}": {:?}"#, name, x)
        } else {
            format!(r#""{}": {}"#, name, x.round())
        };
    }

    format!(r#""{}": "{}""#, name, value)
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
