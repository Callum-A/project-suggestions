use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthProvider {
    Google,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub public_id: String,
    pub email: String,
    pub name: String,
    pub auth_provider: AuthProvider,
}
