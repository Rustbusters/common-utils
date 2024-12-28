
use serde::{Serialize, Deserialize};

use crate::user::UserId;

pub type GroupId = u8;
pub type GroupName = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    id: GroupId,
    name: GroupName,
    members: Vec<UserId>
}
