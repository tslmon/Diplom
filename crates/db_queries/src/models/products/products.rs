use crate::{Aggregation, Crud, ManagementAsyncTrait};
use db_schema::models::errors::PetsShopAPIError;
use db_schema::models::products::{Product, ProductForm};
use db_schema::schema::{products, products::dsl::*};
use db_schema::schema::{user_aggregations, user_aggregations::dsl::*};
use db_schema::ProductId;
use diesel::update;
use diesel::{dsl::*, result::Error, *};
use db_schema::models::{db_error::ModelErrorMessage, model_error::ModelError};
use futures::try_join;

impl Crud<ProductForm, ProductId> for Product {
    fn create(_conn: &PgConnection, _form: &ProductForm) -> Result<Self, ModelError> {
        let _result = insert_into(products)
            .values(_form)
            .get_result::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(PetsShopAPIError::diesel_error(_err)),
        }
    }

    fn read(_conn: &PgConnection, _id: &ProductId) -> Result<Self, ModelError> {
        let _result = products.find(_id).first::<Self>(_conn);

        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(PetsShopAPIError::diesel_error(_err)),
        }
    }

    fn update(
        _conn: &PgConnection,
        _id: &ProductId,
        _form: &ProductForm,
    ) -> Result<Self, ModelError> {
        let _result = diesel::update(products.find(_id))
            .set(_form)
            .get_result::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(PetsShopAPIError::diesel_error(_err)),
        }
    }

    fn update_only(
        _conn: &PgConnection,
        _id: &ProductId,
        _form: &ProductForm,
        _columns: Vec<String>,
    ) -> Result<Self, ModelError> {
        let edited_tenant = _form.clone();
        Self::update(&_conn, _id, &edited_tenant)
    }

    fn delete(_conn: &PgConnection, _id: &ProductId) -> Result<usize, ModelError> {
        let _result = diesel::delete(products.find(_id)).execute(_conn);

        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(PetsShopAPIError::diesel_error(_err)),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl ManagementAsyncTrait<ProductForm, ProductId> for Product {
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
                Self::products_by_filter(&s)
            }
            (Some(q), None) => Self::products_by_filter(q),
            (None, Some(sort)) => {
                s.push_str(" 1=1 ORDER BY ");
                s.push_str(&sort.join(", "));
                Self::products_by_filter(&s)
            }
            (None, None) => products.into_boxed(),
        };
        if let Some(total_count) = _total_count {
            if *total_count {
                let _total_sql = match _q {
                    Some(q) => Self::products_by_filter(q),
                    None => products.into_boxed(),
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
        let mut _result = query.load::<Product>(_conn);

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
        _id: &ProductId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        Product::read(_conn, _id)
    }

    async fn create_item(
        _conn: &PgConnection,
        _form: &ProductForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        Product::create(_conn, _form)
    }

    async fn update_item(
        _conn: &PgConnection,
        _id: &ProductId,
        _form: &ProductForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        Product::update(_conn, _id, _form)
    }

    async fn delete_item(_conn: &PgConnection, _id: &ProductId) -> Result<usize, ModelError> {
        Product::delete(_conn, _id)
    }
}

#[async_trait::async_trait(?Send)]
pub trait Product_ {
    /// Helper functions
    fn products_by_filter<'a>(_sql: &'a str) -> products::BoxedQuery<'a, diesel::pg::Pg> {
        products::table.filter(sql(_sql)).into_boxed()
    }
}

#[async_trait::async_trait(?Send)]
impl Product_ for Product {}
