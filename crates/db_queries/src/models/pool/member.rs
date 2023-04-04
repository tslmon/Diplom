// use db_schema::{models::pool::{members::{Member, MemberForm,MemberData}}, MemberId, schema::pools::{self, dsl::*}, RecruitError};
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::{
    diesel::{ ExpressionMethods},
    Crud, ManagementAsyncTrait,
};
use crate::{ PgConnection};
use db_schema::models::pool::pools::Pool;
use db_schema::schema::pools::{self};
use db_schema::PoolId;
use db_schema::{
    models::pool::members::{Member, MemberData, MemberForm},
    schema::pool_members::{self, dsl::*},
    MemberId, RecruitError,
};
use diesel::dsl::count_star;
use diesel::dsl::sql;
use diesel::insert_into;
use errors_lib_rs::{db::ModelErrorMessage, model::ModelError};
impl Crud<MemberForm, MemberId> for Member {
    fn create(_conn: &PgConnection, _form: &MemberForm) -> Result<Self, ModelError> {
        let _result = diesel::insert_into(pool_members)
            .values(_form)
            .get_result::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }
    fn read(_conn: &PgConnection, _members: &MemberId) -> Result<Self, ModelError> {
        let _result = pool_members
            .filter(pool_members::id.eq(_members.clone()))
            .first::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }
    fn update(
        _conn: &PgConnection,
        _member: &MemberId,
        _form: &MemberForm,
    ) -> Result<Self, ModelError> {
        let _result = diesel::update(pool_members.filter(pool_members::id.eq(_member.clone())))
            .set(_form)
            .get_result::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }
    fn update_only(
        _conn: &PgConnection,
        _id: &MemberId,
        _form: &MemberForm,
        _columns: Vec<String>,
    ) -> Result<Self, ModelError> {
        let edited_member = _form.clone();
        Self::update(&_conn, _id, &edited_member)
    }
    fn delete(_conn: &PgConnection, _id: &MemberId) -> Result<usize, ModelError> {
        let _result = diesel::delete(pool_members.find(_id)).execute(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }
}
#[async_trait::async_trait(?Send)]
impl ManagementAsyncTrait<MemberForm, MemberId> for Member {
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
                Self::member_by_filter(&s)
            }
            (Some(q), None) => Self::member_by_filter(q),
            (None, Some(sort)) => {
                s.push_str(" 1=1 ORDER BY ");
                s.push_str(&sort.join(", "));
                Self::member_by_filter(&s)
            }
            (None, None) => pool_members.into_boxed(),
        };
        if let Some(total_count) = _total_count {
            if *total_count {
                let _total_sql = match _q {
                    Some(q) => Self::member_by_filter(q),
                    None => pool_members.into_boxed(),
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
        let mut _result = query.load::<Member>(_conn);
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
        _id: &MemberId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        Member::read(_conn, _id)
    }
    async fn update_item(
        _conn: &PgConnection,
        _id: &MemberId,
        _form: &MemberForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        Member::update(_conn, _id, _form)
    }
    async fn delete_item(_conn: &PgConnection, _id: &MemberId) -> Result<usize, ModelError> {
        Member::delete(_conn, _id)
    }
}
#[async_trait::async_trait(?Send)]
pub trait Member_ {
    /// Helper functions
    fn member_by_filter<'a>(_sql: &'a str) -> pool_members::BoxedQuery<'a, diesel::pg::Pg> {
        pool_members::table.filter(sql(_sql)).into_boxed()
    }
    async fn delete_items(
        _conn: &PgConnection,
        _pool_id: &PoolId,
        _members: &Vec<String>,
    ) -> Result<usize, ModelError>;

    async fn create_items(
        _conn: &PgConnection,
        _form: &Vec<MemberForm>,
    ) -> Result<Vec<MemberId>, ModelError>
    where
        Self: Sized;

    async fn get_items(
        _conn: &PgConnection,
        _assessment_id: &Vec<MemberId>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _expand: &(bool, bool),
    ) -> Result<Vec<MemberData>, ModelError>
    where
        Self: Sized;
}

#[async_trait::async_trait(?Send)]
impl Member_ for Member {
    async fn create_items(
        _conn: &PgConnection,
        _form: &Vec<MemberForm>,
    ) -> Result<Vec<MemberId>, ModelError>
    where
        Self: Sized,
    {
        let _result = insert_into(pool_members)
            .values(_form)
            .returning(id)
            .get_results::<MemberId>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => return Err(RecruitError::diesel_error(_err)),
        }
    }

    async fn get_items(
        _conn: &PgConnection,
        _id: &Vec<MemberId>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _expand: &(bool, bool),
    ) -> Result<Vec<MemberData>, ModelError>
    where
        MemberData: Sized,
    {
        let _result = pool_members::table
            .inner_join(pools::table)
            .filter(pool_members::id.eq_any(_id))
            .limit(_limit.unwrap_or(10))
            .offset(_offset.unwrap_or(0))
            .load::<(Member, Pool)>(_conn);
        match _result {
            Ok(items) => {
                let mut _member_data = vec![];
                for element in items {
                    let _item = MemberData {
                        id: element.0.id,
                        pool_id: element.0.pool_id,
                        pool_data: element.1,
                        user_role: element.0.user_role,
                        user_id: element.0.user_id,
                        user_data: None,
                        created_by: element.0.created_by,
                        created_at: element.0.created_at,
                        updated_by: element.0.updated_by,
                        updated_at: element.0.updated_at,
                    };
                    _member_data.push(_item);
                }
                Ok(_member_data)
            }
            Err(_err) => return Err(RecruitError::diesel_error(_err)),
        }
    }
    async fn delete_items(
        _conn: &PgConnection,
        _pool_id: &PoolId,
        _users: &Vec<String>,
    ) -> Result<usize, ModelError> {
        let _result = diesel::delete(pool_members)
            .filter(pool_members::pool_id.eq(_pool_id))
            .filter(pool_members::user_id.eq_any(_users))
            .execute(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => return Err(RecruitError::diesel_error(_err)),
        }
    }
}

pub fn get_item_by_member_id(
    _conn: &PgConnection,
    _pool_id: &PoolId,
    _user_id: &String,
) -> Result<MemberId, ModelError> {
    let _result = pool_members
        .select(pool_members::id)
        .filter(pool_id.eq(_pool_id))
        .filter(user_id.eq(_user_id))
        .first::<MemberId>(_conn);
    match _result {
        Ok(res) => Ok(res),
        Err(_err) => return Err(RecruitError::diesel_error(_err)),
    }
}
#[async_trait::async_trait(?Send)]
impl ManagementAsyncTrait<MemberForm, MemberId> for MemberData {}
