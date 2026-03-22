use axum::{
    extract::WebSocketUpgrade,
    response::IntoResponse,
    ws::{ WebSocket, Message },
};
use futures::{ stream::StreamExt, SinkExt };
use tokio::sync::Arc;
use crate::storage::Database;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, db))
}

async fn handle_socket(mut socket: WebSocket, db: Arc<Database>) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(_) = msg {
            // Broadcast recent connections on ping
            if let Ok(connections) = db.get_recent_connections(50).await {
                let json = serde_json::to_string(&connections).unwrap();
                let _ = socket.send(Message::Text(json)).await;
            }
        }
    }
}

