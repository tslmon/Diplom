use crate::{
    schema::{job_candidates, job_candidate_aggregations}, JobId, CandidateId, CandidateAggregationId, StageId,
};
use crate::models::job::stages::Stage;
use serde::Serialize;
use serde_json::Value;
use crate::models::job::jobs::Job;

#[derive(Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Clone, QueryId, Default)]
#[belongs_to(Job)]
#[belongs_to(Stage)]
#[table_name = "job_candidates"]
pub struct Candidate {
    pub id: CandidateId, 
    pub job_id: JobId,
    pub stage_id: StageId,
    pub source_id: String,
    pub user_id: String,
    pub profile_id: String,
    pub name: String,
    pub description: Option<String>,
    pub referred_user_id: String,
    pub originating_candidate_id: String,
    pub disqualified: Option<bool>,
    pub disqualified_by: Option<String>,
    pub disqualified_at: Option<chrono::NaiveDate>,
    pub disqualified_reason: Option<String>,
    pub status: String,
    pub metadata: Value,
    pub app_metadata: Value,
    #[serde(skip)]
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}
#[derive(Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Clone, QueryId)]
#[belongs_to(Candidate)]
#[table_name = "job_candidate_aggregations"]
pub struct CandidateAggregation {
    pub id: CandidateAggregationId,
    pub candidate_id: CandidateId,
    pub events: i64,
    pub offers: i64,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Clone, Default, Debug, Serialize, Queryable)]
#[table_name = "job_candidates"]
pub struct CandidateForm {
    pub user_id: Option<String>,
    pub profile_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub referred_user_id: Option<String>,
    pub metadata: Option<Value>,
    pub app_metadata: Option<Value>,
}