
use db_queries::models::job::stages::Stage_;
use db_queries::{ManagementAsyncTrait};
use db_schema::StageId;
use db_schema::models::job::events::Event;
use db_schema::models::job::job_candidates::Candidate;
use db_schema::models::job::stages::{Stage, StageForm};
use db_schema::models::job::stages::StageAggregation;
use diesel::PgConnection;
use errors_lib_rs::model::ModelError;
use serde::Serialize;
use serde_json::Value;

use crate::ResponceCollection;

#[derive(Serialize, Clone, Debug)]
pub struct StagesView {
    #[serde(flatten)]
    pub item: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregations: Option<StageAggregation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidates: Option<Candidate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    
}
impl StagesView {
    pub async fn create_item(
        _conn: &PgConnection,
        _form: &StageForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<StageId, ModelError> {
        let item = Stage::create_item(_conn, _form, &_fields, _expand).await?;

        Ok(item.id)
    }
    pub async fn update_item(
        _conn: &PgConnection,
        _code: &StageId,
        _form: &StageForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<StageId, ModelError> {
        let item = Stage::update_item(_conn, _code, _form, _fields, _expand).await?;
       
        Ok(item.id)
    }
    pub async fn delete_item(
        _conn: &PgConnection,
        _code: &StageId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<usize, ModelError> {
        Stage::delete_item(_conn, _code).await
    }
    pub async fn get_item(
        _conn: &PgConnection,
        _id: &StageId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError> {
        let _item = Stage::get_item(_conn, _id, &_fields, _expand).await?;

        let _return_item = _item.collect_fields(&_fields).await?;

        let vec = vec![_item.clone()];

        let _childs = Stage::get_stage_childs(_conn, &vec, &_fields, _expand).await?;

        let _default_data = Stage::default();
        let get_expands = Self::get_expands(_expand).await?;
        let vec_id = vec![_item.id];

        let _item = Stage::get_items(_conn, &vec_id, &None, &None,&get_expands).await?;

        let _return_item = _item[0].clone().collect_fields(&_fields).await?;
        Ok(Self {
            item: _return_item,
            aggregations: match &_childs.0 {
                Some(val) => Some(val[0][0].to_owned()),
                None => None,
            },
            candidates: match &_childs.1 {
                Some(val) => Some(val[0][0].to_owned()),
                None => None,
            },
            events: match &_childs.2 {
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

        let (items, _return_count, _has_more) = Stage::get_collection(
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
        let mut _return_list: Vec<StagesView> = Vec::new();

        let mut i: usize = 0;
        let _childs = Stage::get_stage_childs(_conn, &items, &_fields, _expand).await?;
        for item in items.into_iter() {
            let _return_tree = item.collect_fields(_fields).await?;
            _return_list.push(StagesView {
                item: _return_tree,
                aggregations: match &_childs.0 {
                    Some(val) => Some(val[0][0].to_owned()),
                    None => None,
                },
                candidates: match &_childs.1 {
                    Some(val) => Some(val[0][0].to_owned()),
                    None => None,
                },
                events: match &_childs.2 {
                    Some(val) => Some(val[0].to_owned()),
                    None => None,
                },
            });
            i = i + 1;
        }
        let mut _res = ResponceCollection::<StagesView> {
            count: Some(_return_list.len() as i64),
            total_counts: _return_count,
            has_more: _has_more,
            offset: *_offset,
            limit: *_limit,
            items: _return_list,
        };
        Ok(_res)
    }
    async fn get_expands(_expand: &Option<Vec<String>>) -> Result<bool, ModelError> {
        let mut _expands = false;
        if let Some(l_expand) = _expand.clone() {
            _expands = if l_expand.iter().any(|a| a == "all") {
                true
            } else {
                l_expand.iter().any(|a| a == "")
            };
        }

        Ok(_expands)
    }
}