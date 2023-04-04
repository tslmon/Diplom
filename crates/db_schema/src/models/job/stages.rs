use crate::{
    schema::{job_stages, job_stage_aggregations}, JobId, StageId, StageAggregationId
};
use serde::Serialize;
use serde_json::Value;
use crate::models::job::jobs::Job;

#[derive(Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Clone, QueryId, Default)]
#[belongs_to(Job)]
#[table_name = "job_stages"]
pub struct Stage{
    pub id: StageId,
    pub job_id: JobId,
    pub type_: String,
    pub name: String,
    pub description: Option<String>,
    pub sequence: i64,
    pub locked: Option<bool>,
    pub metadata: Value,
    pub app_metadata: Value,
    #[serde(skip)]
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Clone, QueryId)]
#[belongs_to(Stage)]
#[table_name = "job_stage_aggregations"]
pub struct StageAggregation {
    pub id: StageAggregationId,
    pub stage_id: StageId,
    pub candidates: i64,
    pub events: i64,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}
#[derive(Insertable, AsChangeset, Clone, Default, Debug, Serialize)]
#[table_name = "job_stages"]
pub struct StageForm {
    pub job_id: Option<JobId>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
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
#[table_name = "job_stages"]
pub struct StageData {
    pub id: StageId, 
    pub job_id: JobId,
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