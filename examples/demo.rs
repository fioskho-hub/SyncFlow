use syncflow_core::document::Document;
use syncflow_crypto::cipher::Encrypter;
use syncflow_storage::sqlite::MemoryStorage;
use syncflow_storage::StorageEngine;
use syncflow_sync::engine::SyncEngine;
use syncflow_sync::protocol::SyncMessage;

fn main() {
    println!("=== SyncFlow: Démonstration Hors-Ligne & Synchro ===");

    let mut storage_a = MemoryStorage::new();
    let storage_b = MemoryStorage::new();

    let mut doc_a = Document::new("doc-shared");
    doc_a.set_field("title", "Notes de Réunion", 100);
    doc_a.set_field("status", "Brouillon", 100);
    storage_a.save_document(&doc_a).unwrap();
    println!("Nœud A : Document initialisé -> 'Notes de Réunion'");

    let encrypter = Encrypter::new(b"clef_secrete_123");
    let payload_original = b"Payload confidentiel de synchro";
    let _payload_chiffre = encrypter.encrypt_decrypt(payload_original);
    println!("Crypto : Payload chiffré avec succès.");

    let mut engine_a = SyncEngine::new(storage_a);

    let req = SyncMessage::Request {
        document_id: "doc-shared".into(),
    };
    let response = engine_a.handle_message(req);

    if let Some(SyncMessage::Response { document }) = response {
        let mut engine_b = SyncEngine::new(storage_b);

        engine_b.handle_message(SyncMessage::Response {
            document: document.clone(),
        });

        println!("Nœud B : Synchronisation terminée avec succès !");
        println!(
            "Nœud B : Titre reçu -> {:?}",
            document.fields.get("title").unwrap().value
        );
    }
}