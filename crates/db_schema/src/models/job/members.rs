use crate::{
    schema::{job_members}, JobId, MemberId
};
use serde::Serialize;
use crate::models::job::jobs::Job;

#[derive(Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Clone, QueryId, Default)]
#[belongs_to(Job)]
#[table_name = "job_members"]
pub struct Member{
    pub id: MemberId,
    pub job_id: JobId,
    pub user_role: String,
    pub user_id: String,
    #[serde(skip)]
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}