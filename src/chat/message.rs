use crate::user::UserId;
use crate::User;
use serde::{Deserialize, Serialize};

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
    pub content: MessageContent,
    pub timestamp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum MessageContent {
    Text(String),
    // base64 encoded image
    Image(String),
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

    UnregisterSuccess,
    UnregisterFailure,

    ActiveUsersList {
        users: Vec<User>,
    },
    NewUserRegistered {
        id: UserId,
        name: String,
    },
    UserUnregistered {
        id: UserId,
    },

    PrivateMessage {
        sender_id: UserId,
        message: MessageBody,
    },

    UserNotFound {
        user_id: UserId,
    },

    // Error only client side (not for server)
    SendingError {
        message: String,
    },
}
