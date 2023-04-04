use crate::{
    schema::{pool_aggregations, pools},
    PoolAggregationId, PoolId,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(
    Clone, Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Deserialize, Default,
)]
#[table_name = "pools"]
pub struct Pool {
    pub id: PoolId,
    pub name: String,
    pub description: Option<String>,
    pub metadata: Value,
    pub app_metadata: Value,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}
#[derive(Clone, Queryable, Identifiable, Associations, PartialEq, Debug, Serialize)]
#[belongs_to(Pool)]
#[table_name = "pool_aggregations"]
pub struct PoolAggregation {
    pub id: PoolAggregationId,
    pub pool_id: PoolId,
    pub members: i64,
    pub candidates: i64,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}
#[derive(Insertable, AsChangeset, Clone, Default, Debug)]
#[table_name = "pools"]
pub struct PoolForm {
    pub name: Option<String>,
    pub description: Option<String>,  
    pub metadata: Option<Value>, 
    pub app_metadata: Option<Value>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}
#[derive(Insertable, AsChangeset, Clone, Default, Debug, Serialize, Queryable)]
#[table_name = "pools"]
pub struct PoolData {
    pub id: PoolId,
    pub name: String,
    pub description: Option<String>,
    pub metadata: Value,
    pub app_metadata: Value,
    pub created_at: chrono::NaiveDateTime,
    pub created_by: String,
    pub updated_at: chrono::NaiveDateTime,
    pub updated_by: String,
}
