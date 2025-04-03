use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::SharedState;

pub async fn get_websocket_updates(
    State(state): State<SharedState>,
    ws: WebSocketUpgrade,
    Path(id): Path<Uuid>,
) -> axum::response::Response {
    let chans = &state.lock().unwrap().in_progress_channels;
    if let Some(tx) = chans.get(&id) {
        let rx = tx.subscribe(); // Clone receiver for this client
        return ws.on_upgrade(move |socket| handle_websocket(socket, rx));
    }
    axum::http::StatusCode::NOT_FOUND.into_response()
}

async fn handle_websocket(mut socket: WebSocket, mut rx: broadcast::Receiver<serde_json::Value>) {
    while let Ok(status) = rx.recv().await {
        let msg = serde_json::to_string(&status).unwrap();
        if socket.send(Message::Text(msg.into())).await.is_err() {
            break; // Client disconnected
        }
    }
}
