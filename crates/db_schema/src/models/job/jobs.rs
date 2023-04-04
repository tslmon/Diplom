use crate::{
    schema::{jobs, job_aggregations}, JobId, JobAggregationId,
};
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Queryable, Identifiable, PartialEq, Debug, Serialize, Default)]
#[table_name = "jobs"]
pub struct Job {
    pub id: JobId,
    pub name: String,
    pub description: Option<String>,
    pub requirements: String,
    pub workload: String,
    pub temporary: Option<bool>,
    pub function: String,
    pub department: String,
    pub location: String,
    pub industry: String,
    pub benefits: String,
    pub salary: Value,
    pub metadata: Value,
    pub status: String,
    pub app_metadata: Value,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}
#[derive(Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Clone, QueryId)]
#[belongs_to(Job)]
#[table_name = "job_aggregations"]
pub struct JobAggregation {
    pub id: JobAggregationId,
    pub job_id: JobId,
    pub stages: i64,
    pub sources: i64,
    pub candidates: i64,
    pub events: i64,
    pub members: i64,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Clone, Default, Debug, Serialize, Queryable)]
#[table_name = "jobs"]
pub struct JobForm {
    pub name: Option<String>,
    pub description: Option<String>,
    pub requirements: Option<String>,
    pub workload: Option<String>,
    pub temporary: Option<bool>,
    pub function: Option<String>,
    pub department: Option<String>,
    pub location: Option<String>,
    pub industry: Option<String>,
    pub benefits: Option<String>,
    pub salary: Option<Value>,
    pub metadata: Option<Value>,
    pub status: Option<String>,
    pub app_metadata: Option<Value>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}