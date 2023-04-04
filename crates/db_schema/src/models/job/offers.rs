use crate::{
    schema::{job_offers, job_offer_variables, job_offer_documents}, OfferId, JobId, CandidateId, VariableId, DocumentId
};
use chrono::NaiveDate;
use serde::Serialize;
use serde_json::Value;
use crate::models::job::jobs::Job;

#[derive(Queryable, Associations, Identifiable, PartialEq, Debug, Serialize, Clone, QueryId, Default)]
#[belongs_to(Job)]
#[table_name = "job_offers"]
pub struct Offer{
    pub id: OfferId,
    pub job_id: JobId,
    pub candidate_id: CandidateId,
    pub title: String,
    pub body: Option<String>,
    pub status: String,
    pub metadata: Value,
    pub app_metadata: Value,
    #[serde(skip)]
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Associations, PartialEq, Debug, Serialize, Clone, QueryId)]
#[belongs_to(Offer)]
#[table_name = "job_offer_variables"]
pub struct OfferAggregation {
    pub offer_id: OfferId,
    pub variable: VariableId,
    pub value: Option<NaiveDate>,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Associations, PartialEq, Debug, Serialize, Clone, QueryId)]
#[belongs_to(Offer)]
#[table_name = "job_offer_documents"]
pub struct OfferDocument {
    pub id: DocumentId,
    pub offer_id: OfferId,
    pub type_: String,
    pub file_id: String,
    pub file_name: String,
    pub file_type: String,
    pub file_size: Option<i64>,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}