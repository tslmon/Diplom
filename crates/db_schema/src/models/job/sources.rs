use crate::{
    schema::{job_sources}, SourseId, JobId
};
use serde::Serialize;
use serde_json::Value;
use crate::models::job::jobs::Job;

#[derive(Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Clone, QueryId, Default)]
#[belongs_to(Job)]
#[table_name = "job_sources"]
pub struct Sources{
    pub id: SourseId,
    pub job_id: JobId,
    pub type_: String,
    pub name: String,
    pub description: Option<String>,
    pub metadata: Value,
    pub app_metadata: Value,
    #[serde(skip)]
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}
