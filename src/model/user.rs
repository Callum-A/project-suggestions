use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthProvider {
    Google,
}

impl From<String> for AuthProvider {
    fn from(s: String) -> Self {
        match s.as_str() {
            "google" => Self::Google,
            _ => panic!("Unknown provider"),
        }
    }
}

impl AuthProvider {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Google => "google",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub public_id: String,
    pub email: String,
    pub name: String,
    pub auth_provider: AuthProvider,
}
