use serde::{Serialize, Deserialize};

use crate::user::UserId;
use crate::group::{GroupId, GroupName, Group};

// Host Messages sent to Simulation Controller
#[derive(Debug)]
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
#[serde(tag = "type")]
pub enum ClientToServerMessage {
    RegisterUser(String),
    UnregisterUser,

    RequestActiveUsers,

    CreateGroup {
        group_name: GroupName,
        members: Vec<UserId>,
    },
    SearchGroupByName(GroupName),
    JoinGroup(GroupId),

    SendPrivateMessage {
        recipient_id: UserId,
        message: MessageBody,
    },
    SendGroupMessage {
        group_id: GroupId,
        message: MessageBody,
    },
}

// Server -> Client messages
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerToClientMessage {
    RegistrationSuccess,
    RegistrationFailure,

    ActiveUsersList(Vec<UserId>),

    GroupCreated(GroupId),
    GroupDetails(Group), 
    UserJoinedGroup {
        group_id: GroupId,
        user_id: UserId,
    },

    PrivateMessage {
        sender_id: UserId,
        message: MessageBody,
    },
    GroupMessage {
        group_id: GroupId,
        sender_id: UserId,
        message: MessageBody,
    },

    UserNotFound(UserId),
    GroupNotFound(GroupName),
}