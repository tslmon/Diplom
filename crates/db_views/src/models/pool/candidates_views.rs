use crate::get_fields;
use crate::get_keys;
use crate::PgConnection;
use crate::ResponceCollection;
use crate::SetExpand;
use db_queries::models::pool::pool_candidates::Candidate_;
use db_queries::ManagementAsyncTrait;
use db_schema::models::pool::candidates::CandidateData;
use db_schema::models::pool::candidates::CandidateForm;
use db_schema::{models::pool::candidates::Candidate, CandidateId};
use errors_lib_rs::model::ModelError;
use serde::Serialize;
use serde_json::{ Value};

#[derive(Serialize, Clone, Debug)]
pub struct CandidateView {
    #[serde(flatten)]
    pub item: Value,
}
impl CandidateView {
    pub async fn create_item(
        _conn: &PgConnection,
        _form: &CandidateForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<CandidateId, ModelError> {
        let item = Candidate::create_item(_conn, _form, &_fields, &_expand).await?;
        let _return_item = item.collect_fields(&_fields).await?;
        Ok(item.id)
    }
    pub async fn update_item(
        _conn: &PgConnection,
        _id: &CandidateId,
        _form: &CandidateForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<CandidateId, ModelError> {
        let item = Candidate::update_item(_conn, _id, _form, &_fields, _expand).await?;

        let _return_item = item.collect_fields(&_fields).await?;
        Ok(item.id)
    }
    pub async fn delete_item(
        _conn: &PgConnection,
        _id: &CandidateId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<usize, ModelError> {
        Candidate::delete_item(_conn, _id).await
    }
    pub async fn get_item(
        _conn: &PgConnection,
        _id: &CandidateId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError> {
        let _item = Candidate::get_item(_conn, _id, &_fields, _expand).await?;

        let _return_item = _item.collect_fields(&_fields).await?;

        let _default_data = CandidateData::default();

        let _get_expands = Self::get_expands(_expand)?;

        let _item = Candidate::get_items(_conn, &vec![_item.id], &None, &None, &_get_expands.0).await?;

        let json_value: Value = serde_json::to_value(_default_data).unwrap();

        let _data_fields = get_keys(json_value);

        let keys = if _fields.is_some() {
            _fields.to_owned().unwrap()
        } else {
            _data_fields.clone()
        };

        let _expand_vec = vec![_get_expands.0];

        let _get_fields = get_fields(keys, _data_fields, _expand_vec, _get_expands.1).await?;

        let _return_item = _item[0].clone().collect_fields(&_get_fields).await?;

        Ok(Self { item: _return_item })
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

        let (items, _return_count, _has_more) = Candidate::get_collection(
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

        let mut _level_id = vec![];

        for element in items.clone() {
            _level_id.push(element.id)
        }

        let _default_data = CandidateData::default();

        let _get_expands = Self::get_expands(_expand)?;

        let _item = Candidate::get_items(_conn, &_level_id, &None, &None, &_get_expands.0).await?;

        let mut _return_list: Vec<CandidateView> = Vec::new();

        let json_value: Value = serde_json::to_value(_default_data).unwrap();

        let _data_fields = get_keys(json_value);

        let keys = if _fields.is_some() {
            _fields.to_owned().unwrap()
        } else {
            _data_fields.clone()
        };

        let _expand_vec = vec![_get_expands.0];

        let _get_fields = get_fields(keys, _data_fields, _expand_vec, _get_expands.1).await?;

        let mut i: usize = 0;

        for item in _item.into_iter() {
            let _return_candidate = item.collect_fields(&_get_fields).await?;

            _return_list.push(CandidateView{
                item: _return_candidate,
            });
            i = i + 1;
        }

        let mut _res = ResponceCollection::<CandidateView> {
            count: Some(_return_list.len() as i64),
            total_counts: _return_count,
            has_more: _has_more,
            offset: *_offset,
            limit: *_limit,
            items: _return_list,
        };

        Ok(_res)
    }
    fn get_expands(
        _expand: &Option<Vec<String>>,
    ) -> Result<(bool, Vec<SetExpand>), ModelError> {
        let mut _expands = false;
        if let Some(l_expand) = _expand.clone() {
            _expands = if l_expand.iter().any(|a| a == "all") {
                true
            } else {
                    l_expand.iter().any(|a| a == "candidate_data")
                
            };
        }

        let mut _expand_arr: Vec<SetExpand> = Vec::new();

        _expand_arr.push(SetExpand(2, String::from("candidate_data")));
        Ok((_expands, _expand_arr))
    }
}
