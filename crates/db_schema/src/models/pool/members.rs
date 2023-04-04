use crate::models::pool::pools::Pool;
use crate::{schema::pool_members, MemberId, PoolId};
use serde::{Serialize};
#[derive(Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Clone, QueryId, Default)]
#[belongs_to(Pool)]
#[table_name = "pool_members"]
pub struct Member {
    pub id: MemberId,
    pub pool_id: PoolId,
    pub user_role: String,
    pub user_id: String,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}
#[derive(Insertable, AsChangeset, Clone, Default, Debug, Serialize, Queryable)]
#[table_name = "pool_members"]
pub struct MemberForm {
    pub pool_id :Option<PoolId>,
    pub user_id:Option<String>,
    pub user_role: Option<String>,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Clone, QueryId, Default)]
#[belongs_to(Pool)]
#[table_name = "pool_members"]
pub struct MemberData {   
    pub id: MemberId,
    pub pool_id: PoolId,
    pub pool_data: Pool,
    pub user_role: String,
    pub user_id: String,
    pub user_data: Option<String>,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}
