use serde::{Deserialize, Serialize};

/// 角色修改
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoleEditDTO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub auths: Vec<String>,
    pub parent_id: Option<String>,
}