use crate::actions::ManagementTrait;
use crate::common::{
    connection, user::*, CollectionRequest, LoginApi, LoginRequest, SingularRequest,
};
use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    web::Json,
    Error, FromRequest, HttpRequest, HttpResponse, Responder,
};
use db_queries::models::users::users::User_;
use db_schema::models::model_error::{ApiError, ApiErrorEnum};
use db_schema::{models::users::*, UserId};
use db_schema::{
    models::users::{User, UserForm},
    naive_now,
};
use db_views::users::users_view::UserView;
use db_views::RequestCollection;
use futures::future::{ok, Ready};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::Value;
use utils::claims::{self, Claims};

#[async_trait::async_trait(?Send)]
impl ManagementTrait<LoginRequest> for LoginApi {
    type Response = HttpResponse;

    async fn create_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<LoginRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);
        //println!("{:#?}", _data);

        let b = db_queries::login_user(
            &_conn,
            _data.user_name.clone().unwrap(),
            _data.password.clone().unwrap(),
        )?;
        let mut a = String::new();
        a = Claims::jwt(
            _data.user_name.clone().unwrap(),
            _data.password.clone().unwrap(),
        )
        .unwrap();
        if b.user_name == _data.user_name.clone().unwrap() {
            

        }
        println!("{:#?}", a);
        let _body = serde_json::to_string(&a)?;
        Ok(HttpResponse::Created()
            .content_type(ContentType::json())
            .body(_body))
    }
}
