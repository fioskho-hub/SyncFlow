use crate::protocol::SyncMessage;
use syncflow_core::document::Document;
use syncflow_storage::StorageEngine;

pub struct SyncEngine<S: StorageEngine> {
    storage: S,
}

impl<S: StorageEngine> SyncEngine<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub fn handle_message(&mut self, message: SyncMessage) -> Option<SyncMessage> {
        match message {
            SyncMessage::Request { document_id } => {
                if let Ok(Some(doc)) = self.storage.get_document(&document_id) {
                    Some(SyncMessage::Response { document: doc })
                } else {
                    None
                }
            }
            SyncMessage::Response { document } => {
                if let Ok(local_doc) = self.storage.get_document(&document.id) {
                    let mut updated_doc = local_doc.unwrap_or_else(|| Document::new(&document.id));
                    updated_doc.merge(&document);
                    let _ = self.storage.save_document(&updated_doc);
                }
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syncflow_storage::sqlite::MemoryStorage;

    #[test]
    fn test_sync_engine_request_response() {
        let mut storage = MemoryStorage::new();
        let mut doc = Document::new("doc-sync");
        doc.set_field("status", "synced", 100);
        storage.save_document(&doc).unwrap();

        let mut engine = SyncEngine::new(storage);
        let response = engine.handle_message(SyncMessage::Request {
            document_id: "doc-sync".into(),
        });

        assert!(matches!(response, Some(SyncMessage::Response { .. })));
    }
}