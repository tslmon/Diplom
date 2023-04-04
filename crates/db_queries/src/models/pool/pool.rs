use db_schema::{models::pool::{pools::{Pool, PoolForm, PoolAggregation, PoolData}, candidates::Candidate, members::Member}, PoolId, schema::pools::{self, dsl::*}, RecruitError};
use crate::{
    diesel::{BelongingToDsl, ExpressionMethods, GroupedBy},
    Crud, ManagementAsyncTrait,
};

use diesel::{
    dsl::{count_star, sql}, QueryDsl, RunQueryDsl, PgConnection,
};
use errors_lib_rs::{db::ModelErrorMessage, model::ModelError};
use futures::try_join;

impl Crud<PoolForm, PoolId> for Pool {
    fn create(_conn: &PgConnection, _form: &PoolForm) -> Result<Self, ModelError> {
        let _result = diesel::insert_into(pools)
            .values(_form)
            .get_result::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }

    fn read(_conn: &PgConnection, _id: &PoolId) -> Result<Pool, ModelError> {
        let _result = pools.find(_id).first::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }
    fn update(
        _conn: &PgConnection,
        _id: &PoolId,
        _form: &PoolForm,
    ) -> Result<Self, ModelError> {
        let _result = diesel::update(pools.find(_id))
            .set(_form)
            .get_result::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }

    fn update_only(
        _conn: &PgConnection,
        _id: &PoolId,
        _form: &PoolForm,
        _columns: Vec<String>,
    ) -> Result<Self, ModelError> {
        let edited_pool = _form.clone();
        Self::update(&_conn, _id, &edited_pool)
    }
    fn delete(_conn: &PgConnection, _id: &PoolId) -> Result<usize, ModelError> {
        let _result = diesel::delete(pools.find(_id)).execute(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }
}
#[async_trait::async_trait(?Send)]
impl ManagementAsyncTrait<PoolForm, PoolId> for Pool {
    async fn get_collection(
        _conn: &PgConnection,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
        _q: &Option<String>,
        _sort: &Option<Vec<String>>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _total_count: &Option<bool>,
    ) -> Result<(Vec<Self>, Option<i64>, bool), ModelError>
    where
        Self: Sized,
    {
        let mut l_return_count: Option<i64> = None;
        let mut s = String::new();
        let mut query = match (_q, _sort) {
            (Some(q), Some(sort)) => {
                s.push_str(q);
                s.push_str(" ORDER BY ");
                s.push_str(&sort.join(", "));
                Self::pools_by_filter(&s)
            }
            (Some(q), None) => Self::pools_by_filter(q),
            (None, Some(sort)) => {
                s.push_str(" 1=1 ORDER BY ");
                s.push_str(&sort.join(", "));
                Self::pools_by_filter(&s)
            }
            (None, None) => pools.into_boxed(),
        };
        if let Some(total_count) = _total_count {
            if *total_count {
                let _total_sql = match _q {
                    Some(q) => Self::pools_by_filter(q),
                    None => pools.into_boxed(),
                };
                let _result = _total_sql.select(count_star()).first::<i64>(_conn);
                match _result {
                    Ok(res) => l_return_count = Some(res),
                    Err(_err) => return Err(RecruitError::diesel_error(_err)),
                }
            }
        }
        if let Some(offset) = _offset {
            query = query.offset(*offset);
        }
        if let Some(limit) = _limit {
            query = query.limit(*limit + 1);
        }
        let mut _result = query.load::<Pool>(_conn);
        let mut _data = match _result {
            Ok(res) => res,
            Err(_err) => return Err(RecruitError::diesel_error(_err)),
        };
        let l_has_more = _data.len() > _limit.unwrap() as usize;
        let mut _l_limit: usize;
        if l_has_more {
            _l_limit = _limit.unwrap() as usize;
            _data = (_data[.._l_limit]).to_vec();
        }
        Ok((_data, l_return_count, l_has_more))
    }
    async fn get_item(
        _conn: &PgConnection,
        _id: &PoolId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        Pool::read(_conn, &_id)
    }
    

    async fn create_item(
        _conn: &PgConnection,
        _form: &PoolForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        Pool::create(_conn, _form)
    }
    async fn update_item(
        _conn: &PgConnection,
        _id: &PoolId,
        _form: &PoolForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        Pool::update(_conn, _id, _form)
    }
    async fn delete_item(_conn: &PgConnection, _id: &PoolId) -> Result<usize, ModelError> {
        Pool::delete(_conn, _id)
    }
}
#[async_trait::async_trait(?Send)]
pub trait Pool_ {
    fn pools_by_filter<'a>(_sql: &'a str) -> pools::BoxedQuery<'a, diesel::pg::Pg> {
        pools::table.filter(sql(_sql)).into_boxed()
    }
    async fn get_items(
        _conn: &PgConnection,
        _pools_id: &Vec<PoolId>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _expand: &bool,
    ) -> Result<Vec<PoolData>, ModelError>
    where
        Self: Sized;
async fn get_pool_childs(
        _conn: &PgConnection,
        _pools: &Vec<Pool>,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<
        (
            Option<Vec<Vec<PoolAggregation>>>,
            Option<Vec<Vec<Member>>>,
            Option<Vec<Vec<Candidate>>>,
        ),
        ModelError,
    >
    where
        Self: Sized;
    async fn get_pool_aggregations(
        _conn: &PgConnection,
        _pools: &Vec<Pool>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<PoolAggregation>>>, ModelError>
    where
    Self: Sized;
    async fn get_pool_member(
        _conn: &PgConnection,
        _clients: &Vec<Pool>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
         _expand: &bool,
    ) -> Result<Option<Vec<Vec<Member>>>, ModelError>
    
    where
            Self: Sized; 
async fn get_pool_candidate(
    _conn: &PgConnection,
    _clients: &Vec<Pool>,
    _offset: &Option<i64>,
    _limit: &Option<i64>,
    _fields: &Option<Vec<String>>,
    _expand: &bool,
    ) -> Result<Option<Vec<Vec<Candidate>>>, ModelError>
    where
    Self: Sized;  
}
#[async_trait::async_trait(?Send)]
        impl Pool_ for Pool{
            async fn get_items(
                _conn: &PgConnection,
                _pools_id: &Vec<PoolId>,
                _offset: &Option<i64>,
                _limit: &Option<i64>,
                _expand: &bool,
            ) -> Result<Vec<PoolData>, ModelError>
            where
                Self: Sized,
            {
                let _result = pools::table
            .filter(pools::id.eq_any(_pools_id))
            .limit(_limit.unwrap_or(10))
            .offset(_offset.unwrap_or(0))
            .load::<Pool>(_conn);
        match _result {
            Ok(items) => {
                let mut _pool_data = vec![];
                for element in items {
                    let _item = PoolData {
                        id: element.id,
                        name: element.name,
                        description: element.description,
                        metadata: element.metadata,
                        app_metadata: element.app_metadata,
                        created_at: element.created_at,
                        updated_at: element.updated_at,
                        created_by: element.created_by,
                        updated_by: element.updated_by
                    };
                    _pool_data.push(_item);
                }

                Ok(_pool_data)
            }
            Err(_err) => return Err(RecruitError::diesel_error(_err)),
        }
    }
    async fn get_pool_aggregations(
        _conn: &PgConnection,
        _pools: &Vec<Pool>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<PoolAggregation>>>, ModelError>
    where
        Self: Sized,
    {
        if *_expand {
            let _result = PoolAggregation::belonging_to(_pools)
                .limit(_limit.unwrap_or(10))
                .load::<PoolAggregation>(_conn);
            match _result {
                Ok(_aggregations) => {
                    let _grouped_aggregations: Vec<Vec<PoolAggregation>> =
                        _aggregations.grouped_by(_pools);
                    Ok(Some(_grouped_aggregations))
                }
                Err(_err) => return Err(RecruitError::diesel_error(_err)),
            }
        } else {
            Ok(None)
        }
    }
    async fn get_pool_member(
        _conn: &PgConnection,
        _pools: &Vec<Pool>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<Member>>>, ModelError>
    where
        Self: Sized,
    {
        if *_expand {
            let _result = Member::belonging_to(_pools).load::<Member>(_conn);

            match _result {
                Ok(topics) => {
                    let _grouped_topics: Vec<Vec<Member>> = topics.grouped_by(_pools);
                    Ok(Some(_grouped_topics))
                }
                Err(_err) => return Err(RecruitError::diesel_error(_err)),
            }
        } else {
            Ok(None)
        }
    }
    async fn get_pool_candidate(
        _conn: &PgConnection,
        _pools: &Vec<Pool>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<Candidate>>>, ModelError>
    where
        Self: Sized,
    {
        if *_expand {
            let _result = Candidate::belonging_to(_pools).load::<Candidate>(_conn);

            match _result {
                Ok(topics) => {
                    let _grouped_topics: Vec<Vec<Candidate>> = topics.grouped_by(_pools);
                    Ok(Some(_grouped_topics))
                }
                Err(_err) => return Err(RecruitError::diesel_error(_err)),
            }
        } else {
            Ok(None)
        }
    }
    async fn get_pool_childs(
        _conn: &PgConnection,
        _pools: &Vec<Pool>,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<
        (
            Option<Vec<Vec<PoolAggregation>>>,
            Option<Vec<Vec<Member>>>,
            Option<Vec<Vec<Candidate>>>,
        ),
        ModelError,
    > {
        let _return = match _expand {
            Some(val) => {
                let exec_plan: (bool, bool, bool) =
                    if val.iter().any(|a| a == "all") {
                        (true, true, true)
                    } else {
                        (
                            val.iter().any(|a| a == "aggregations"),
                            val.iter().any(|a| a == "members"),
                            val.iter().any(|a| a == "candidates"),
                        )
                    };
                    let aggregations_fut = Pool::get_pool_aggregations(
                        _conn,
                        _pools,
                        &None,
                        &None,
                        &None,
                        &exec_plan.0,
                    );
                    let member_fut = Pool::get_pool_member(
                        _conn,
                        _pools,
                        &None,
                        &None,
                        &None,
                        &exec_plan.1,
                    );
                    let association_fut = Pool::get_pool_candidate(
                        _conn,
                        _pools,
                        &None,
                        &None,
                        &None,
                        &exec_plan.2,
                    );
                    try_join!(
                        aggregations_fut,
                        member_fut,
                        association_fut,
                    )?
                }
                None => (None, None, None),
            };
            Ok(_return)
        }
    }
    #[async_trait::async_trait(?Send)]
    impl ManagementAsyncTrait<PoolForm, PoolId> for PoolData {

    }