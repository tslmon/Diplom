#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_newtype;
pub mod models;
pub mod schema;
use chrono::NaiveDateTime;
use errors_lib_rs::db::ModelErrorMessage;
use serde::{Deserialize, Serialize};
use std::fmt;

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

pub fn naive_now() -> NaiveDateTime {
    chrono::prelude::Utc::now().naive_utc()
}
#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct UserId(pub String);
impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
pub struct RecruitError {}

impl ModelErrorMessage for RecruitError {
    fn uniqueviolation_message(_message: Option<&str>) -> Option<String> {
        if _message.is_some() {
            match _message.unwrap() {
                "ukey_job_name" => Some(String::from("Job name already exists.")),
                "ukey_pool_name" => Some(String::from("Pool name already exists.")),
                _ => todo!(),
            }
        } else {
            None
        }
    }
    fn foreignkeyviolation_message(_message: Option<&str>) -> Option<String> {
        if _message.is_some() {
            match _message.unwrap() {
                "fkey_job_sources_jobs" => Some(String::from("Source id already exists.")),
                "fkey_job_stages_jobs" => Some(String::from("Stage id already exists.")),
                "fkey_job_candidates_jobs" => Some(String::from("Cabdidate id already exists.")),
                "fkey_job_candidates_job_stages" => Some(String::from("Stage id already exists.")),
                "fkey_job_candidates_job_sources" => Some(String::from("Source id already exists.")),
                "fkey_job_offers_jobs" => Some(String::from("Job id already exists.")),
                "fkey_job_offers_job_candidates" => Some(String::from("Candidate id already exists.")),
                "fkey_job_members_jobs" => Some(String::from("Job id already exists.")),
                "fkey_job_events_jobs" => Some(String::from("Source id already exists.")),
                "fkey_job_events_job_stages" => Some(String::from("Stage id already exists.")),
                "fkey_job_offer_variables_job_offers" => Some(String::from("Offer id already exists.")),
                "fkey_job_offer_documents_job_offers" => Some(String::from("Offer id already exists.")),
                "fkey_job_event_candidates_job_events" => Some(String::from("Event id already exists.")),
                "fkey_job_event_candidates_job_candidates" => Some(String::from("Candidate id already exists.")),
                "fkey_job_event_organizers_job_events" => Some(String::from("Event id already exists.")),
                "fkey_job_event_organizers_job_members" => Some(String::from("Member id already exists.")),
                "fkey_job_aggregations_jobs" => Some(String::from("Job id already exists.")),
                "fkey_job_source_aggregations_job_sources" => Some(String::from("Source id already exists.")),
                "fkey_job_stage_aggregations_job_stages" => Some(String::from("Stage id already exists.")),
                "fkey_job_offer_aggregations_job_offers" => Some(String::from("Offer id already exists.")),
                "fkey_job_candidate_aggregations_job_candidates" => Some(String::from("Candidate id already exists.")),
                "fkey_job_event_aggregations_job_events" => Some(String::from("Event id already exists.")),
                "fkey_pool_candidates_pools" => Some(String::from("Pool Candidates id already exists.")),
                _ => todo!(),
            }
        } else {
            None
        }
    }
}
