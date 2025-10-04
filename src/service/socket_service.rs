use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use tracing::info;

// ------------------- Structs --------------------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectAuth {
    pub user_id: String,
    pub user_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Offer {
    pub sdp: String,
    pub type_: String,
    pub sender_id: String,
    pub receiver_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Answer {
    pub sdp: String,
    pub type_: String,
    pub sender_id: String,
    pub receiver_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Candidate {
    pub candidate: String,
    pub sdpMid: String,
    pub sdpMLineIndex: u32,
    pub usernameFragment: String,
    pub sender_id: String,
    pub receiver_id: String,
}

// ------------------- Global Store --------------------
static USER_INFO: Lazy<DashMap<String, ConnectAuth>> = Lazy::new(DashMap::new);
static USER_SOCKET: Lazy<DashMap<String, String>> = Lazy::new(DashMap::new);

// ------------------- Socket Handlers --------------------
pub async fn on_connect(socket: SocketRef, Data(auth): Data<ConnectAuth>, io: SocketIo) {
    let socket_id = socket.id.to_string();
    let user_id = auth.user_id.clone();

    // Save user ↔ socket mapping
    USER_SOCKET.insert(user_id.clone(), socket_id.clone());
    USER_INFO.insert(user_id.clone(), auth.clone());

    println!("🟢 Socket connected: {} (user_id={})", socket_id, user_id);
    info!("ℹ️ User {} connected", user_id);

    let _ = socket.emit("welcome", &format!("Hello user {}", user_id));

    // ---------- OFFER ----------
    let io_offer = io.clone();
    socket.on("offer", move |_: SocketRef, Data(offer): Data<Offer>| {
        let io_offer = io_offer.clone();
        tokio::spawn(async move {
            if let Some(target_socket_id) = USER_SOCKET.get(&offer.receiver_id) {
                let target = target_socket_id.value().clone();
                if let Err(e) = io_offer.to(target).emit("offer", &offer).await {
                    eprintln!("❌ Failed to send offer: {:?}", e);
                }
                println!("📤 Sent offer from {} -> {}", offer.sender_id, offer.receiver_id);
            }
        });
    });

    // ---------- ANSWER ----------
    let io_answer = io.clone();
    socket.on("answer", move |_: SocketRef, Data(answer): Data<Answer>| {
        let io_answer = io_answer.clone();
        tokio::spawn(async move {
            if let Some(target_socket_id) = USER_SOCKET.get(&answer.receiver_id) {
                let target = target_socket_id.value().clone();
                if let Err(e) = io_answer.to(target).emit("answer", &answer).await {
                    eprintln!("❌ Failed to send answer: {:?}", e);
                }
                println!("📤 Sent answer from {} -> {}", answer.sender_id, answer.receiver_id);
            }
        });
    });

    // ---------- ICE CANDIDATE ----------
    let io_ice = io.clone();
    socket.on("ice-candidate", move |_: SocketRef, Data(candidate): Data<Candidate>| {
        let io_ice = io_ice.clone();
        tokio::spawn(async move {
            if let Some(target_socket_id) = USER_SOCKET.get(&candidate.receiver_id) {
                let target = target_socket_id.value().clone();
                if let Err(e) = io_ice.to(target).emit("ice-candidate", &candidate).await {
                    eprintln!("❌ Failed to send ICE: {:?}", e);
                }
                println!(
                    "📤 Sent ICE from {} -> {}",
                    candidate.sender_id, candidate.receiver_id
                );
            }
        });
    });

    // ---------- DISCONNECT ----------
    socket.on_disconnect(move |socket: SocketRef| {
        let socket_id = socket.id.to_string();
        if let Some(entry) = USER_SOCKET.iter().find(|kv| kv.value() == &socket_id) {
            let user_id = entry.key().clone();
            USER_INFO.remove(&user_id);
            USER_SOCKET.remove(&user_id);

            println!("🔴 User {} disconnected (socket_id={})", user_id, socket_id);
        }
    });
}
