pub mod client_to_server;
pub mod server_to_client;

use serde::{Deserialize, Serialize};
use wg_2024::network::NodeId;

pub type GroupId = u8;
pub type GroupName = String;

pub trait HostMessage: Clone + std::fmt::Debug + Serialize + for<'de> Deserialize<'de> {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageBody {
    pub src: NodeId,
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
