use crate::{Aggregation, Crud, ManagementAsyncTrait};
use db_schema::models::errors::PetsShopAPIError;
use db_schema::models::users::{User, UserForm};
use db_schema::schema::{user_aggregations, user_aggregations::dsl::*};
use db_schema::schema::{users, users::dsl::*};
use db_schema::UserId;
use diesel::update;
use diesel::{dsl::*, result::Error, *};
use errors_lib_rs::{db::ModelErrorMessage, model::ModelError};
use futures::try_join;

impl Crud<UserForm, UserId> for User {
    fn create(_conn: &PgConnection, _form: &UserForm) -> Result<Self, ModelError> {
        let _result = insert_into(users).values(_form).get_result::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(PetsShopAPIError::diesel_error(_err)),
        }
    }

    fn read(_conn: &PgConnection, _id: &UserId) -> Result<Self, ModelError> {
        let _result = users.find(_id).first::<Self>(_conn);

        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(PetsShopAPIError::diesel_error(_err)),
        }
    }

    fn update(_conn: &PgConnection, _id: &UserId, _form: &UserForm) -> Result<Self, ModelError> {
        let _result = diesel::update(users.find(_id))
            .set(_form)
            .get_result::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(PetsShopAPIError::diesel_error(_err)),
        }
    }

    fn update_only(
        _conn: &PgConnection,
        _id: &UserId,
        _form: &UserForm,
        _columns: Vec<String>,
    ) -> Result<Self, ModelError> {
        let edited_tenant = _form.clone();
        Self::update(&_conn, _id, &edited_tenant)
    }

    fn delete(_conn: &PgConnection, _id: &UserId) -> Result<usize, ModelError> {
        let _result = diesel::delete(users.find(_id)).execute(_conn);

        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(PetsShopAPIError::diesel_error(_err)),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl ManagementAsyncTrait<UserForm, UserId> for User {
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
                Self::users_by_filter(&s)
            }
            (Some(q), None) => Self::users_by_filter(q),
            (None, Some(sort)) => {
                s.push_str(" 1=1 ORDER BY ");
                s.push_str(&sort.join(", "));
                Self::users_by_filter(&s)
            }
            (None, None) => users.into_boxed(),
        };
        if let Some(total_count) = _total_count {
            if *total_count {
                let _total_sql = match _q {
                    Some(q) => Self::users_by_filter(q),
                    None => users.into_boxed(),
                };
                let _result = _total_sql.select(count_star()).first::<i64>(_conn);
                match _result {
                    Ok(res) => l_return_count = Some(res),
                    Err(_err) => return Err(PetsShopAPIError::diesel_error(_err)),
                }
            }
        }
        if let Some(offset) = _offset {
            query = query.offset(*offset);
        }
        if let Some(limit) = _limit {
            query = query.limit(*limit + 1);
        }
        let mut _result = query.load::<User>(_conn);

        let mut _result = match _result {
            Ok(res) => res,
            Err(_err) => return Err(PetsShopAPIError::diesel_error(_err)),
        };

        let l_has_more = _result.len() > _limit.unwrap() as usize;
        let mut _l_limit: usize;
        if l_has_more {
            _l_limit = _limit.unwrap() as usize;
            _result = (_result[.._l_limit]).to_vec();
        }
        Ok((_result, l_return_count, l_has_more))
    }

    async fn get_item(
        _conn: &PgConnection,
        _id: &UserId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        User::read(_conn, _id)
    }

    async fn create_item(
        _conn: &PgConnection,
        _form: &UserForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        User::create(_conn, _form)
    }

    async fn update_item(
        _conn: &PgConnection,
        _id: &UserId,
        _form: &UserForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        User::update(_conn, _id, _form)
    }

    async fn delete_item(_conn: &PgConnection, _id: &UserId) -> Result<usize, ModelError> {
        User::delete(_conn, _id)
    }
}

/// CRUD implementation for Tenant Identity Provider
///
/// Create, Update, Delete, Read operation implementation
/// fn create(
///     _conn: &PgConnection,
///     _form: &UserScopeForm
/// ) -> Result<Self, ModelError>
///

// impl Aggregation<UserId> for UserAggregation {
//     fn read(_conn: &PgConnection, _id: UserId) -> Result<Self, ModelError> {
//         let _result = User_aggregations
//             .filter(User_aggregations::User_id.eq(_id))
//             .first::<Self>(_conn);
//         match _result {
//             Ok(res) => Ok(res),
//             Err(_err) => Err(PetsShopAPIError::diesel_error(_err)),
//         }
//     }
// }

#[async_trait::async_trait(?Send)]
pub trait User_ {
    /// Helper functions
    fn users_by_filter<'a>(_sql: &'a str) -> users::BoxedQuery<'a, diesel::pg::Pg> {
        users::table.filter(sql(_sql)).into_boxed()
    }
    // async fn set_secret(
    //     _conn: &PgConnection,
    //     _user_id: UserId,
    //     _user_secret: &str,
    //     _user_secret_expires_at: chrono::NaiveDateTime,
    // ) -> Result<Client, ModelError>
    // where
    //     Self: Sized;

    async fn get_user_childs(
        _conn: &PgConnection,
        _user: &Vec<User>,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<(Option<Vec<Vec<UserAggregation>>>, Option<Vec<Vec<Room>>>), ModelError>
    where
        Self: Sized;

    async fn get_user_aggregations(
        _conn: &PgConnection,
        _user: &Vec<User>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<UserAggregation>>>, ModelError>
    where
        Self: Sized;

    async fn get_user_grouped_rooms(
        _conn: &PgConnection,
        _user: &Vec<User>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<Room>>>, ModelError>
    where
        Self: Sized;
}

#[async_trait::async_trait(?Send)]
impl User_ for User {
    // async fn set_secret(
    //     _conn: &PgConnection,
    //     _user_id: UserId,
    //     _user_secret: &str,
    //     _user_secret_expires_at: chrono::NaiveDateTime,
    // ) -> Result<Client, ModelError>
    // where
    //     Self: Sized,
    // {
    //     let a = Users::update(Users.filter(id.eq(_user_id)))
    //         .set((
    //             client_secret.eq(_user_secret),
    //             client_secret_expires_at.eq(_user_secret_expires_at),
    //         ))
    //         .get_result::<Self>(_conn);
    //     Ok(a.unwrap())
    // }
    async fn get_user_aggregations(
        _conn: &PgConnection,
        _user: &Vec<User>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<UserAggregation>>>, ModelError>
    where
        Self: Sized,
    {
        if *_expand {
            let _aggrigations = UserAggregation::belonging_to(_user)
                .limit(_limit.unwrap_or(10))
                .load::<UserAggregation>(_conn);

            let _aggrigations = match _aggrigations {
                Ok(res) => res,
                Err(_err) => return Err(PetsShopAPIError::diesel_error(_err)),
            };

            let _grouped_aggrigations: Vec<Vec<UserAggregation>> = _aggrigations.grouped_by(_user);
            Ok(Some(_grouped_aggrigations))
        } else {
            Ok(None)
        }
    }

    async fn get_user_grouped_rooms(
        _conn: &PgConnection,
        _user: &Vec<User>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<Room>>>, ModelError>
    where
        Self: Sized,
    {
        if *_expand {
            let _rooms = Room::belonging_to(_user)
                .limit(_limit.unwrap_or(10))
                .load::<Room>(_conn);

            let _rooms = match _rooms {
                Ok(res) => res,
                Err(_err) => return Err(PetsShopAPIError::diesel_error(_err)),
            };

            let _grouped_rooms: Vec<Vec<Room>> = _rooms.grouped_by(_user);
            Ok(Some(_grouped_rooms))
        } else {
            Ok(None)
        }
    }

    async fn get_user_childs(
        _conn: &PgConnection,
        _user: &Vec<User>,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<(Option<Vec<Vec<UserAggregation>>>, Option<Vec<Vec<Room>>>), ModelError> {
        let _return = match _expand {
            Some(val) => {
                let exec_plan: (bool, bool) = if val.iter().any(|a| a == "all") {
                    (true, true)
                } else {
                    (
                        val.iter().any(|a| a == "aggregations"),
                        val.iter().any(|a| a == "rooms"),
                    )
                };
                let aggregations_fut =
                    User::get_user_aggregations(_conn, _user, &None, &None, &None, &exec_plan.0);

                let rooms_fut =
                    User::get_user_grouped_rooms(_conn, _user, &None, &None, &None, &exec_plan.1);

                try_join!(aggregations_fut, rooms_fut)?
            }
            None => (None, None),
        };
        Ok(_return)
    }
}
