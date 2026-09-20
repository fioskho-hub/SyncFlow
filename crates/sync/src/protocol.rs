use serde::{Deserialize, Serialize};
use syncflow_core::document::Document;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncMessage {
    Request { document_id: String },
    Response { document: Document },
}