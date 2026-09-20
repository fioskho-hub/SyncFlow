use syncflow_core::document::Document;

pub trait StorageEngine {
    type Error;

    fn save_document(&mut self, doc: &Document) -> Result<(), Self::Error>;

    fn get_document(&self, id: &str) -> Result<Option<Document>, Self::Error>;
}

pub mod sqlite;