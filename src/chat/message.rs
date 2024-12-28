use serde::{Deserialize, Serialize};

use crate::user::UserId;

// Host Messages sent to Simulation Controller
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "message")]
pub enum HostMessage {
    FromClient(ClientToServerMessage),
    FromServer(ServerToClientMessage),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageBody {
    pub sender_id: UserId,
    pub message: MessageContent,
    pub timestamp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum MessageContent {
    Text(String),
    Image(Vec<u8>),
    Audio(Vec<u8>),
}

// Client -> Server messages
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "request")]
pub enum ClientToServerMessage {
    RegisterUser {
        name: String,
    },
    UnregisterUser,

    RequestActiveUsers,

    SendPrivateMessage {
        recipient_id: UserId,
        message: MessageBody,
    },
}

// Server -> Client messages
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "response")]
pub enum ServerToClientMessage {
    RegistrationSuccess,
    RegistrationFailure,

    ActiveUsersList {
        users: Vec<UserId>,
    },

    PrivateMessage {
        sender_id: UserId,
        message: MessageBody,
    },

    UserNotFound {
        user_id: UserId,
    },
}
