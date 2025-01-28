use serde::{Deserialize, Serialize};
use wg_2024::network::NodeId;

pub type UserId = NodeId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    name: String,
}

impl User {
    pub fn new(id: UserId, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    
}