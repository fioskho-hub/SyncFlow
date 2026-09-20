use crate::crdt::LwwRegister;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Document {
    pub id: String,
    pub fields: HashMap<String, LwwRegister<String>>,
}

impl Document {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            fields: HashMap::new(),
        }
    }

    pub fn set_field(&mut self, key: impl Into<String>, value: impl Into<String>, timestamp: u64) {
        let key = key.into();
        let new_register = LwwRegister::new(value.into(), timestamp);

        self.fields
            .entry(key)
            .and_modify(|reg| {
                reg.merge(new_register.clone());
            })
            .or_insert(new_register);
    }

    pub fn merge(&mut self, other: &Document) {
        for (key, other_reg) in &other.fields {
            self.fields
                .entry(key.clone())
                .and_modify(|local_reg| {
                    local_reg.merge(other_reg.clone());
                })
                .or_insert_with(|| other_reg.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_merge() {
        let mut doc1 = Document::new("doc-1");
        doc1.set_field("title", "Titre V1", 100);

        let mut doc2 = Document::new("doc-1");
        doc2.set_field("title", "Titre V2", 200);
        doc2.set_field("author", "Gabri", 150);

        doc1.merge(&doc2);

        assert_eq!(doc1.fields.get("title").unwrap().value, "Titre V2");
        assert_eq!(doc1.fields.get("author").unwrap().value, "Gabri");
    }
}