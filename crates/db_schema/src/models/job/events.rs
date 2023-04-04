use crate::{
    schema::{job_events, job_event_organizers, job_event_candidates, job_event_aggregations}, JobId, EventId, EventAggregationId, OrganizerId, MemberId, CandidateId, EventCandidateId, StageId,
};
use serde::Serialize;
use serde_json::Value;
use crate::models::job::jobs::Job;

use super::stages::Stage;

#[derive(Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Clone, QueryId, Default)]
#[belongs_to(Job)]
#[belongs_to(Stage)]
#[table_name = "job_events"]
pub struct Event {
    pub id: EventId, 
    pub job_id: JobId,
    pub stage_id: StageId,
    pub type_: String,
    pub name: String,
    pub description: Option<String>,
    pub sequence: i64,
    pub metadata: Value,
    pub app_metadata: Value,
    #[serde(skip)]
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}
#[derive(Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Clone, QueryId)]
#[belongs_to(Event)]
#[table_name = "job_event_aggregations"]
pub struct EventAggregation {
    pub id: EventAggregationId,
    pub event_id: EventId,
    pub organizers: i64,
    pub candidates: i64,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Clone, QueryId)]
#[belongs_to(Event)]
#[table_name = "job_event_organizers"]
pub struct Organizer {
    pub id: OrganizerId,
    pub event_id: EventId,
    pub member_id: MemberId,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Clone, QueryId)]
#[belongs_to(Event)]
#[table_name = "job_event_candidates"]
pub struct EventCandidate {
    pub id: EventCandidateId,
    pub event_id: EventId,
    pub candidate_id: CandidateId,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Clone, Default, Debug, Serialize)]
#[table_name = "job_events"]
pub struct EventForm {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub stage_id : Option<StageId>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub sequence: Option<i64>,
    pub metadata: Option<Value>,
    pub app_metadata: Option<Value>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Clone, QueryId, Default)]
#[belongs_to(Job)]
#[table_name = "job_events"]
pub struct EventData {
    pub id: EventId, 
    pub job_id: JobId,
    pub job_data: Job,
    pub stage_id: StageId,
    pub stage_data: Option<Stage>,
    pub type_: String,
    pub name: String,
    pub description: Option<String>,
    pub sequence: i64,
    pub metadata: Value,
    pub app_metadata: Value,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}