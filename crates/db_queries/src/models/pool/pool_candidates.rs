use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::{
    diesel::{ ExpressionMethods,},
    Crud, ManagementAsyncTrait,
};
use crate::{PgConnection};
use db_schema::models::pool::pools::Pool;
// use db_schema::PoolId;
use db_schema::{
    models::pool::candidates::{Candidate, CandidateData, CandidateForm},
    schema::pool_candidates::{self, dsl::*},
    CandidateId, RecruitError,
};
use diesel::dsl::count_star;
use diesel::dsl::sql;
use errors_lib_rs::{db::ModelErrorMessage, model::ModelError};
use db_schema::schema::pools::dsl::pools;
impl Crud<CandidateForm, CandidateId> for Candidate {
    fn create(_conn: &PgConnection, _form: & CandidateForm) -> Result<Self, ModelError> {
        let _result = diesel::insert_into(pool_candidates)
            .values(_form)
            .get_result::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }

    fn read(_conn: &PgConnection, _id: & CandidateId) -> Result< Candidate , ModelError> {
        let _result = pool_candidates.find(_id).first::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }
    fn update(
        _conn: &PgConnection,
        _id: &CandidateId,
        _form: &CandidateForm,
    ) -> Result<Self, ModelError> {
        let _result = diesel::update(pool_candidates.find(_id))
            .set(_form)
            .get_result::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }
    fn update_only(
        _conn: &PgConnection,
        _id: &CandidateId, 
        _form: &CandidateForm,
        _columns: Vec<String>,
    ) -> Result<Self, ModelError> {
        let edited_candidate_cri = _form.clone();
        Self::update(&_conn, _id, &edited_candidate_cri)
    }
    fn delete(_conn: &PgConnection, _id: &CandidateId) -> Result<usize, ModelError> {
        let _result = diesel::delete(pool_candidates.find(_id)).execute(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }

}

#[async_trait::async_trait(?Send)]
impl ManagementAsyncTrait<CandidateForm, CandidateId> for Candidate {
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
                Self::pool_candidates_by_filter(&s)
            }
            (Some(q), None) => Self::pool_candidates_by_filter(q),
            (None, Some(sort)) => {
                s.push_str(" 1=1 ORDER BY ");
                s.push_str(&sort.join(", "));
                Self::pool_candidates_by_filter(&s)
            }
            (None, None) => pool_candidates.into_boxed(),
        };
        if let Some(total_count) = _total_count {
            if *total_count {
                let _total_sql = match _q {
                    Some(q) => Self::pool_candidates_by_filter(q),
                    None => pool_candidates.into_boxed(),
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
        let mut _result = query.load::<Candidate>(_conn);
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
        _id: &CandidateId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        Candidate::read(_conn, &_id)
    }

    async fn create_item(
        _conn: &PgConnection,
        _form: &CandidateForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        Candidate::create(_conn, _form)
    }
    async fn update_item(
        _conn: &PgConnection,
        _id: &CandidateId,
        _form: &CandidateForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        Candidate::update(_conn, _id, _form)
    }
    async fn delete_item(_conn: &PgConnection, _id: &CandidateId) -> Result<usize, ModelError> {
        Candidate::delete(_conn, _id)
    }
}
#[async_trait::async_trait(?Send)]
pub trait Candidate_ {
    fn pool_candidates_by_filter<'a>(_sql: &'a str) -> pool_candidates::BoxedQuery<'a, diesel::pg::Pg> {
       pool_candidates::table.filter(sql(_sql)).into_boxed()
    }
    async fn get_items(
        _conn: &PgConnection,
        _id: &Vec<CandidateId>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _expand: &bool,
    ) -> Result<Vec<CandidateData>, ModelError>
    where
        Self: Sized;
}
#[async_trait::async_trait(?Send)]
impl Candidate_ for Candidate {
async fn get_items(
    _conn: &PgConnection,
    _candidate_id: &Vec<CandidateId>,
    _offset: &Option<i64>,
    _limit: &Option<i64>,
    _expand: &bool,
) -> Result<Vec<CandidateData>, ModelError>
where
    Self: Sized,
{
    let _result = pool_candidates::table
        .inner_join(pools)
        .filter(pool_candidates::id.eq_any(_candidate_id))
        .limit(_limit.unwrap_or(10))
        .offset(_offset.unwrap_or(0))
        .load::<(Candidate,Pool)>(_conn);
    match _result {
        Ok(items) => {
            let mut _candidate_data = vec![];
            for element in items {
                let _item = CandidateData {
                id: element.0.id,
                pool_id: element.0.pool_id,
                pool_data: element.1,
                user_id: element.0.user_id,
                user_data: None,
                profile_id: element.0.profile_id,
                profile_data: None,
                name: element.0.name,
                description:element.0.description,
                originating_candidate_id: element.0.originating_candidate_id,
                originating_candidate_data: None,
                metadata: element.0.metadata,
                app_metadata: element.0.app_metadata,
                created_at: element.0.created_at,
                updated_at: element.0.updated_at,
                created_by: element.0.created_by,
                updated_by: element.0.updated_by,
                };
                _candidate_data.push(_item);
            }

            Ok(_candidate_data)
        }
        Err(_err) => return Err(RecruitError::diesel_error(_err)),
    }
}
}

#[async_trait::async_trait(?Send)]
impl ManagementAsyncTrait<CandidateForm, CandidateId> for CandidateData {}
