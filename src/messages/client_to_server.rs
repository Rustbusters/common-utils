use crate::{GroupId, GroupName, HostMessage, MessageBody};
use serde::{Deserialize, Serialize};
use wg_2024::network::NodeId;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageToServer {
    Register,
    Unregister,

    GetActiveUsers,

    CreateGroup {
        name: GroupName,
        members: Vec<NodeId>,
    },
    SearchGroup {
        name: GroupName,
    },
    EnterGroup {
        group_id: GroupId,
    },

    SendToUser {
        user_id: NodeId,
        message: MessageBody,
    },
    SendToGroup {
        group_id: GroupId,
        message: MessageBody,
    },
}
impl HostMessage for MessageToServer {}
