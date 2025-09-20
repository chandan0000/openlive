use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde::Deserialize;
use socketioxide::extract::{Data, SocketRef};
use std::sync::Mutex;
use tracing::info;

#[derive(Debug, Deserialize, Clone)]
pub struct ConnectAuth {
    user_id: String,
    user_type: String,
}

static USER_INFO: Lazy<Mutex<DashMap<String, ConnectAuth>>> =
    Lazy::new(|| Mutex::new(DashMap::new()));

pub async fn on_connect(socket: SocketRef, Data(auth): Data<ConnectAuth>) {
    let socket_id = socket.id.to_string();
    let user_id = auth.user_id.clone();

    {
        let user_info = USER_INFO.lock().unwrap();
        user_info.insert(socket_id.clone(), auth.clone());
    }

    println!("🟢 [PRINT] Socket connected: {}", socket_id);
    println!(
        "🟢 [PRINT] Auth received: user_id={}, user_type={}",
        user_id, auth.user_type
    );
    info!("ℹ️ [LOG] User {} connected", user_id);

    let _ = socket.emit("welcome", &format!("Hello user {}", user_id));

    socket.on_disconnect(move |socket: SocketRef| {
        println!("🔴 [PRINT] Socket disconnected: {}", socket.id);
        let socket_id = socket.id.to_string();
        let user_info = USER_INFO.lock().unwrap();
        user_info.remove(&socket_id);
    });
}
