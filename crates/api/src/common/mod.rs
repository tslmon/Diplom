pub mod job;
pub mod pool;
use actix_web::{web::Data, HttpRequest};
use db_queries::DbPool;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use utils::AuthError;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SingularRequest {
    pub fields: Option<Vec<String>>,
    pub expand: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct CollectionRequest {
    pub fields: Option<Vec<String>>,
    pub expand: Option<Vec<String>>,
    pub q: Option<String>,
    pub sort: Option<Vec<String>>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub total_count: Option<bool>,
}

pub async fn blocking<F, T>(pool: &Option<DbPool>, f: F) -> Result<T, AuthError>
where
    F: FnOnce(&diesel::PgConnection) -> T + Send + 'static,
    T: Send + 'static,
{
    let pool = pool.clone();
    let res = actix_web::web::block(move || {
        let conn = pool.unwrap().get()?;
        let res = (f)(&conn);
        Ok(res) as Result<_, AuthError>
    })
    .await?;

    res
}
pub fn connection(req: &HttpRequest) -> PooledConnection<ConnectionManager<PgConnection>> {
    let auth_context: Option<&Data<AuthContext>> = req.app_data::<Data<AuthContext>>();

    let v = auth_context.unwrap().as_ref().pool().clone().unwrap();
    let conn = v.get().unwrap();
    conn
}
pub struct AuthContext {
    pub pool: Option<DbPool>,
    pub client: Client,
}
impl AuthContext {
    pub fn create(pool: Option<DbPool>, client: Client) -> AuthContext {
        AuthContext { pool, client }
    }
    pub fn pool(&self) -> &Option<DbPool> {
        &self.pool
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
}
