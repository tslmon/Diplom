
use crate::get_fields;
use crate::get_keys;
use crate::PgConnection;
use crate::ResponceCollection;
use crate::SetExpand;
use db_queries::ManagementAsyncTrait;
use db_queries::models::pool::pool::Pool_;
use db_schema::PoolId;
use db_schema::models::pool::pools::Pool;
use db_schema::models::pool::pools::PoolAggregation;
use db_schema::models::pool::members::Member;
use db_schema::models::pool::candidates::Candidate;
use db_schema::models::pool::pools::PoolForm;
use db_schema::models::pool::pools::PoolData;
use errors_lib_rs::model::ModelError;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use db_queries::ViewToVec;

#[derive(Serialize, Clone, Debug)]
pub struct PoolView {
    #[serde(flatten)]
    pub item: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregations: Option<PoolAggregation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidates: Option<Vec<Candidate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
}
type PoolViewTuple = (
    (Pool, PoolAggregation),
    (
        Vec<Candidate>,
        Vec<Member>,
    ),
);
impl ViewToVec for PoolView {
    type DbTuple = PoolViewTuple;
    fn from_tuple_to_vec(items: Vec<Self::DbTuple>) -> Vec<Self> {
        items
            .iter()
            .map(|a| Self {
                item: json!(a.0 .0.to_owned()),
                aggregations: Some(a.0 .1.to_owned()),
                candidates: Some(a.1 .0.to_owned()),                
                members: Some(a.1 .1.to_owned()),
            })
            .collect::<Vec<Self>>()
    }
}
impl PoolView {
    pub async fn create_item(
        _conn: &PgConnection,
        _form: &PoolForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<PoolId, ModelError> {
        let item = Pool::create_item(_conn, _form, &_fields, &_expand).await?;

        Ok(item.id)
    }
    pub async fn update_item(
        _conn: &PgConnection,
        _id: &PoolId,
        _form: &PoolForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<PoolId, ModelError> {
        let item = Pool::update_item(_conn, _id, _form, &_fields, _expand).await?;
        Ok(item.id)
    }
    pub async fn delete_item(
        _conn: &PgConnection,
        _id: &PoolId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<usize, ModelError> {
        Pool::delete_item(_conn, _id).await
    }
    pub async fn get_item(
        _conn: &PgConnection,
        _id: &PoolId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError> {
        let item = Pool::get_item(&_conn, &_id, _fields, _expand).await?;
        let _return_item = item.collect_fields(&_fields).await?;
        let vec = vec![item.clone()];

        let _childs = Pool::get_pool_childs(_conn, &vec, &_fields, _expand).await?;

        let _default_data = PoolData::default();

        let get_expands = Self::get_expands(_expand).await?;

        let vec_id = vec![item.id];

        let _item = Pool::get_items(_conn, &vec_id, &None, &None, &get_expands).await?;

        let json_value: Value = serde_json::to_value(_default_data).unwrap();

        let _data_fields = get_keys(json_value);

        let keys = if _fields.is_some() {
            _fields.to_owned().unwrap()
        } else {
            _data_fields.clone()
        };
        let _expand_vec = vec![false];

        let _set_expand = vec![SetExpand(2, String::from(""))];

        let _get_fields = get_fields(keys, _data_fields, _expand_vec, _set_expand).await?;

        let _return_item = _item[0].clone().collect_fields(&_get_fields).await?;
        Ok(Self {
            item: _return_item,
            aggregations: match &_childs.0 {
                Some(val) => Some(val[0][0].to_owned()),
                None => None,
            },
            members: match &_childs.1 {
                Some(val) => Some(val[0].to_owned()),
                None => None,
            },
            candidates: match &_childs.2 {
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

        let (items, _return_count, _has_more) = Pool::get_collection(
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

        let _childs = Pool::get_pool_childs(_conn, &items, &None, _expand).await?;
        let mut _pools_id = vec![];

        for element in items.clone() {
            _pools_id.push(element.id)
        }

        let _default_data =PoolData::default();

        let _get_expands = Self::get_expands(_expand).await?;

        let _item = Pool::get_items(_conn, &_pools_id, &None, &None, &_get_expands).await?;

        let json_value: Value = serde_json::to_value(_default_data).unwrap();

        let _data_fields = get_keys(json_value);

        let keys = if _fields.is_some() {
            _fields.to_owned().unwrap()
        } else {
            _data_fields.clone()
        };
        let _expand_vec = vec![false];

        let _set_expand = vec![SetExpand(2, String::from(""))];

        let _get_fields = get_fields(keys, _data_fields, _expand_vec, _set_expand).await?;

        let mut _return_list: Vec<PoolView> = Vec::new();

        let mut i: usize = 0;

        for item in _item.into_iter() {
            let _return_pool = item.clone().collect_fields(&_get_fields).await?;
            _return_list.push(PoolView {
                item: _return_pool,
                aggregations: match &_childs.0 {
                    Some(val) => Some(val[i][0].to_owned()),
                    None => None,
                },
                members: match &_childs.1 {
                    Some(val) => Some(val[0].to_owned()),
                    None => None,
                },
                candidates: match &_childs.2 {
                    Some(val) => Some(val[0].to_owned()),
                    None => None,
                },
            });

            i = i + 1;
        }

        let mut _res = ResponceCollection::<PoolView> {
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