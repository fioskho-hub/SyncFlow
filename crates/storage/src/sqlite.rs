use crate::StorageEngine;
use syncflow_core::document::Document;
use std::collections::HashMap;

pub struct MemoryStorage {
    docs: HashMap<String, Document>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            docs: HashMap::new(),
        }
    }
}

impl StorageEngine for MemoryStorage {
    type Error = String;

    fn save_document(&mut self, doc: &Document) -> Result<(), Self::Error> {
        self.docs.insert(doc.id.clone(), doc.clone());
        Ok(())
    }

    fn get_document(&self, id: &str) -> Result<Option<Document>, Self::Error> {
        Ok(self.docs.get(id).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_storage_save_and_get() {
        let mut storage = MemoryStorage::new();
        let mut doc = Document::new("doc-test");
        doc.set_field("title", "Test Storage", 100);

        storage.save_document(&doc).unwrap();

        let retrieved = storage.get_document("doc-test").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().fields.get("title").unwrap().value, "Test Storage");
    }
}