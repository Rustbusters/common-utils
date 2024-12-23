use crate::{GroupId, GroupName, HostMessage, MessageBody};
use serde::{Deserialize, Serialize};
use wg_2024::network::NodeId;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageToClient {
    Registered,
    RegisterFailure,

    ActiveUsers {
        users: Vec<NodeId>,
    },

    GroupCreated {
        group_id: GroupId,
    },
    GroupFound {
        group_id: GroupId,
        name: GroupName,
        members: Vec<NodeId>,
    },
    UserJoined {
        group_id: GroupId,
        user: NodeId,
    },

    MessageFrom {
        src: NodeId,
        message: MessageBody,
    },
    MessageFromGroup {
        group_id: GroupId,
        src: NodeId,
        message: MessageBody,
    },

    UserNotFound {
        user: NodeId,
    },
    GroupNotFound {
        name: GroupName,
    },
}
impl HostMessage for MessageToClient {}
