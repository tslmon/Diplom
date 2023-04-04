use serde::{Deserialize, Serialize};
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
pub struct MemberApi {}

#[derive(Serialize, Deserialize)]
pub struct JobCollection {}

#[derive(Serialize, Deserialize, Clone)]
pub struct MemberRequest {
    pub user_role: Option<String>,
    pub users: Vec<String>,
}
