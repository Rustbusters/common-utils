use serde::{Deserialize, Serialize};
use wg_2024::network::NodeId;

pub type UserId = NodeId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    name: String,
}
