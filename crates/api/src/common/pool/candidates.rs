use db_schema::PoolId;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use serde_json::Value;
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
pub struct CandidateApi {}

#[derive(Serialize, Deserialize)]
pub struct JobCollection {}

#[derive(Serialize, Deserialize, Clone)]
pub struct CandidateRequest {
    pub user_id:Option<String>,
    pub profile_id:Option<String>,
    pub name:Option<String>,
    pub description:Option<String>,
    pub originating_candidate_id:Option<String>,
    pub metadata:Option<Value>,
    pub app_metadata:Option<Value>,
}
pub struct CandidateMoveApi {}

#[derive(Serialize, Deserialize, Clone)]
pub struct CandidateMoveRequest {
    pub pool_id: Option<PoolId>,
}
pub struct CandidateDuplicateApi {}
#[derive(Serialize, Deserialize, Clone)]
pub struct CandidateDuplicateRequest {
    pub pool_id: Option<PoolId>,
}