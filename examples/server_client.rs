use syncflow_core::document::Document;
use syncflow_storage::sqlite::SqliteStorage;
use syncflow_storage::StorageEngine;
use syncflow_sync::engine::SyncEngine;
use syncflow_sync::protocol::SyncMessage;
use syncflow_sync::transport::{TransportClient, TransportServer};
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080";
    let ws_url = "ws://127.0.0.1:8080";

    let mut server_storage = SqliteStorage::open(":memory:")?;
    
    let mut doc_server = Document::new("doc-1");
    doc_server.set_field("title", "Titre Serveur", 100);
    server_storage.save_document(&doc_server)?;

    let server_engine = Arc::new(Mutex::new(SyncEngine::new(server_storage)));
    let engine_for_server = server_engine.clone();

    tokio::spawn(async move {
        println!("[Serveur] Écoute sur {}", addr);
        let _ = TransportServer::listen(addr, move |req_msg| {
            println!("[Serveur] Message reçu du client : {:?}", req_msg);
            let mut engine = engine_for_server.lock().unwrap();
            engine.handle_message(req_msg)
        })
        .await;
    });

    sleep(Duration::from_millis(200)).await;

    let mut client_storage = SqliteStorage::open(":memory:")?;

    let mut doc_client = Document::new("doc-1");
    doc_client.set_field("client_note", "Note Client local", 150);
    client_storage.save_document(&doc_client)?;

    let mut client_engine = SyncEngine::new(client_storage);

    println!("\n[Client] Envoi de la requête de synchro au serveur...");
    let sync_request = SyncMessage::Request {
        document_id: "doc-1".to_string(),
    };

    if let Some(response_msg) = TransportClient::send(ws_url, sync_request).await? {
        println!("[Client] Réponse reçue du serveur : {:?}", response_msg);
        client_engine.handle_message(response_msg);
        println!("[Client] Synchronisation terminée !");
    }

    println!("\n=== Synchro effectuée avec succès ! ===");

    Ok(())
}