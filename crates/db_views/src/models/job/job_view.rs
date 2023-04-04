use crate::{PgConnection, ResponceCollection};
use db_queries::models::job::jobs::Job_;
use db_queries::{ManagementAsyncTrait};
use db_schema::JobId;
use db_schema::models::job::events::Event;
use db_schema::models::job::job_candidates::Candidate;
use db_schema::models::job::jobs::Job;
use db_schema::models::job::jobs::JobAggregation;
use db_schema::models::job::jobs::JobForm;
use db_schema::models::job::members::Member;
use db_schema::models::job::sources::Sources;
use db_schema::models::job::stages::Stage;
use errors_lib_rs::model::ModelError;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Clone, Debug)]
pub struct JobView {
    #[serde(flatten)]
    pub item: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregations: Option<JobAggregation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stages: Option<Vec<Stage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Sources>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidates: Option<Vec<Candidate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
}
impl JobView {
    pub async fn create_item(
        _conn: &PgConnection,
        _form: &JobForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<JobId, ModelError> {
        let item = Job::create_item(_conn, _form, &_fields, _expand).await?;

        Ok(item.id)
    }
    pub async fn update_item(
        _conn: &PgConnection,
        _code: &JobId,
        _form: &JobForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<JobId, ModelError> {
        let item = Job::update_item(_conn, _code, _form, _fields, _expand).await?;
       
        Ok(item.id)
    }
    pub async fn delete_item(
        _conn: &PgConnection,
        _code: &JobId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<usize, ModelError> {
        Job::delete_item(_conn, _code).await
    }
    pub async fn get_item(
        _conn: &PgConnection,
        _id: &JobId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError> {
        let _item = Job::get_item(_conn, _id, &_fields, _expand).await?;

        let _return_item = _item.collect_fields(&_fields).await?;

        let vec = vec![_item.clone()];

        let _childs = Job::get_job_childs(_conn, &vec, &_fields, _expand).await?;

        let _default_data = Job::default();

        let vec_id = vec![_item.id];

        let _item = Job::get_items(_conn, &vec_id, &None, &None).await?;

        let _return_item = _item[0].clone().collect_fields(&_fields).await?;
        Ok(Self {
            item: _return_item,
            aggregations: match &_childs.0 {
                Some(val) => Some(val[0][0].to_owned()),
                None => None,
            },
            stages: match &_childs.1 {
                Some(val) => Some(val[0].to_owned()),
                None => None,
            },
            sources: match &_childs.2 {
                Some(val) => Some(val[0].to_owned()),
                None => None,
            },
            candidates: match &_childs.3 {
                Some(val) => Some(val[0].to_owned()),
                None => None,
            },
            events: match &_childs.4 {
                Some(val) => Some(val[0].to_owned()),
                None => None,
            },
            members: match &_childs.5 {
                Some(val) => Some(val[0].to_owned()),
                None => None,
            },
        })
    }
    pub async fn get_collection(
        _conn: &PgConnection,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
        _q: &Option<String>,
        _sort: &Option<Vec<String>>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _total_count: &Option<bool>,
    ) -> Result<ResponceCollection<Self>, ModelError> {
        let mut _return_count: Option<i64>;

        let (items, _return_count, _has_more) = Job::get_collection(
            _conn,
            _fields,
            _expand,
            _q,
            _sort,
            _offset,
            _limit,
            _total_count,
        )
        .await?;
        let mut _return_list: Vec<JobView> = Vec::new();

        let mut i: usize = 0;
        let _childs = Job::get_job_childs(_conn, &items, &_fields, _expand).await?;
        for item in items.into_iter() {
            let _return_tree = item.collect_fields(_fields).await?;
            _return_list.push(JobView {
                item: _return_tree,
                aggregations: match &_childs.0 {
                    Some(val) => Some(val[0][0].to_owned()),
                    None => None,
                },
                stages: match &_childs.1 {
                    Some(val) => Some(val[0].to_owned()),
                    None => None,
                },
                sources: match &_childs.2 {
                    Some(val) => Some(val[0].to_owned()),
                    None => None,
                },
                candidates: match &_childs.3 {
                    Some(val) => Some(val[0].to_owned()),
                    None => None,
                },
                events: match &_childs.4 {
                    Some(val) => Some(val[0].to_owned()),
                    None => None,
                },
                members: match &_childs.5 {
                    Some(val) => Some(val[0].to_owned()),
                    None => None,
                },
            });
            i = i + 1;
        }
        let mut _res = ResponceCollection::<JobView> {
            count: Some(_return_list.len() as i64),
            total_counts: _return_count,
            has_more: _has_more,
            offset: *_offset,
            limit: *_limit,
            items: _return_list,
        };
        Ok(_res)
    }
}
