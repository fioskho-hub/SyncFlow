use crate::protocol::SyncMessage;
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, connect_async, tungstenite::Message};

pub struct TransportServer;

impl TransportServer {
    pub async fn listen<F>(addr: &str, mut handler: F) -> Result<(), String>
    where
        F: FnMut(SyncMessage) -> Option<SyncMessage> + Send + 'static,
    {
        let listener = TcpListener::bind(addr).await.map_err(|e| e.to_string())?;

        while let Ok((stream, _)) = listener.accept().await {
            if let Ok(mut ws_stream) = accept_async(stream).await {
                while let Some(Ok(msg)) = ws_stream.next().await {
                    if let Message::Text(text) = msg {
                        if let Ok(sync_msg) = serde_json::from_str::<SyncMessage>(&text) {
                            if let Some(response) = handler(sync_msg) {
                                let resp_text = serde_json::to_string(&response).unwrap();
                                let _ = ws_stream.send(Message::Text(resp_text)).await;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

pub struct TransportClient;

impl TransportClient {
    pub async fn send(url: &str, msg: SyncMessage) -> Result<Option<SyncMessage>, String> {
        let (mut ws_stream, _) = connect_async(url).await.map_err(|e| e.to_string())?;
        
        let json_text = serde_json::to_string(&msg).map_err(|e| e.to_string())?;
        ws_stream
            .send(Message::Text(json_text))
            .await
            .map_err(|e| e.to_string())?;

        if let Some(Ok(Message::Text(text))) = ws_stream.next().await {
            let response: SyncMessage = serde_json::from_str(&text).map_err(|e| e.to_string())?;
            Ok(Some(response))
        } else {
            Ok(None)
        }
    }
}