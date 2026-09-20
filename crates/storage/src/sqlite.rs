use crate::StorageEngine;
use rusqlite::{params, Connection};
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

pub struct SqliteStorage {
    conn: Connection,
}

impl SqliteStorage {
    pub fn open(path: &str) -> Result<Self, String> {
        let conn = Connection::open(path).map_err(|e| e.to_string())?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                data TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| e.to_string())?;

        Ok(Self { conn })
    }
}

impl StorageEngine for SqliteStorage {
    type Error = String;

    fn save_document(&mut self, doc: &Document) -> Result<(), Self::Error> {
        let json_data = serde_json::to_string(doc).map_err(|e| e.to_string())?;

        self.conn
            .execute(
                "INSERT INTO documents (id, data) VALUES (?1, ?2)
                 ON CONFLICT(id) DO UPDATE SET data = excluded.data",
                params![doc.id, json_data],
            )
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn get_document(&self, id: &str) -> Result<Option<Document>, Self::Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT data FROM documents WHERE id = ?1")
            .map_err(|e| e.to_string())?;

        let mut rows = stmt.query(params![id]).map_err(|e| e.to_string())?;

        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let json_data: String = row.get(0).map_err(|e| e.to_string())?;
            let doc: Document = serde_json::from_str(&json_data).map_err(|e| e.to_string())?;
            Ok(Some(doc))
        } else {
            Ok(None)
        }
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

    #[test]
    fn test_sqlite_storage_save_and_get() {
        let mut storage = SqliteStorage::open(":memory:").unwrap();
        let mut doc = Document::new("doc-sqlite");
        doc.set_field("title", "Persistance SQLite", 100);

        storage.save_document(&doc).unwrap();

        let retrieved = storage.get_document("doc-sqlite").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().fields.get("title").unwrap().value, "Persistance SQLite");
    }
}