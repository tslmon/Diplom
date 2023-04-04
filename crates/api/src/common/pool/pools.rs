use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IdentityProvider {
    Aircampi,
    Apple,
    Facebook,
    Github,
    Google,
    Twitter,
}

impl Display for IdentityProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match &self {
            Self::Aircampi => write!(f, "aircampi"),
            Self::Apple => write!(f, "apple"),
            Self::Facebook => write!(f, "facebook"),
            Self::Github => write!(f, "github"),
            Self::Google => write!(f, "google"),
            Self::Twitter => write!(f, "twitter"),
        }
    }
}
pub struct PoolApi {}

#[derive(Serialize, Deserialize)]
pub struct JobCollection {}

#[derive(Serialize, Deserialize, Clone)]
pub struct PoolRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<Value>,
    pub app_metadata: Option<Value>,
}
//
// Status
//
#[derive(Serialize, Deserialize, Clone)]
pub struct PoolStatusRequest {
    pub status: Option<String>
}

#[derive(Serialize)]
pub struct PoolStatusApi {}
