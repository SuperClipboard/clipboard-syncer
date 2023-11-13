use graphql_client::GraphQLQuery;

use crate::graphql::record_by_md5_query::{
    RecordByMd5QueryAllRecord002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099f,
    RecordByMd5QueryAllRecord002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099fDocuments,
    RecordByMd5QueryAllRecord002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099fDocumentsFields,
    RecordByMd5QueryAllRecord002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099fDocumentsMeta,
};
use crate::graphql::record_by_pages::{
    record_002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099fOrderBy,
    RecordByPagesAllRecord002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099fDocuments,
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/next_args.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct NextArgsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/publish.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct PublishMut;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/record.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct RecordByMd5Query;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/record.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct RecordCounts;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/record.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct RecordByPages;

pub type GraphRecord =
    RecordByMd5QueryAllRecord002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099f;

pub type GraphRecordDocuments = RecordByMd5QueryAllRecord002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099fDocuments;

pub type GraphRecordDocumentsFields = RecordByMd5QueryAllRecord002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099fDocumentsFields;

pub type GraphRecordDocumentsMeta = RecordByMd5QueryAllRecord002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099fDocumentsMeta;

pub type GraphRecordPageDocuments = RecordByPagesAllRecord002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099fDocuments;

pub type GraphRecordOrderBy =
    record_002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099fOrderBy;

pub type DocumentId = p2panda_rs::document::DocumentId;

pub type DocumentViewId = String;

pub type EntryHash = p2panda_rs::hash::Hash;

pub type LogId = p2panda_rs::entry::LogId;

pub type PublicKey = p2panda_rs::identity::PublicKey;

pub type SeqNum = p2panda_rs::entry::SeqNum;

pub type EncodedEntry = p2panda_rs::entry::EncodedEntry;

pub type EncodedOperation = p2panda_rs::operation::EncodedOperation;

pub type Cursor = String;
