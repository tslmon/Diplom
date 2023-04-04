use crate::{
    schema::{pool_candidates}, PoolId, CandidateId, 
};
use serde::Serialize;
use serde_json::Value;
use crate::models::pool::pools::Pool;

#[derive(Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Clone, QueryId, Default)]
#[belongs_to(Pool)]
#[table_name = "pool_candidates"]
pub struct Candidate{
    pub id: CandidateId,
    pub pool_id: PoolId,
    pub user_id:String,
    pub profile_id:String,
    pub name:String,
    pub description:Option<String>,
    pub originating_candidate_id: String,
    pub metadata:Value,
    pub app_metadata:Value,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,

}

#[derive(Insertable, AsChangeset, Clone, Default, Debug, Serialize, Queryable)]
#[table_name = "pool_candidates"]
pub struct CandidateForm {
    pub pool_id: Option<PoolId>,
    pub user_id:Option<String>,
    pub profile_id:Option<String>,
    pub name:Option<String>,
    pub description:Option<String>,
    pub originating_candidate_id:Option<String>,
    pub metadata:Option<Value>,
    pub app_metadata:Option<Value>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}
#[derive(Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Clone, QueryId, Default)]
#[belongs_to(Pool)]
#[table_name = "pool_candidates"]
pub struct CandidateData{
    pub id: CandidateId,
    pub pool_id: PoolId,
    pub pool_data: Pool,
    pub user_id:String,
    pub user_data: Option<String>,
    pub profile_id:String,
    pub profile_data:Option<String>,
    pub name:String,
    pub description:Option<String>,
    pub originating_candidate_id: String,
    pub originating_candidate_data: Option<String>,
    pub metadata:Value,
    pub app_metadata:Value,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,

}
