use crate::actions::job::ManagementTrait;
use crate::common::pool::members::{MemberApi, MemberRequest};
use crate::common::{connection, CollectionRequest, SingularRequest};
use actix_web::http::header::ContentType;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse};
use db_queries::models::pool::member;
use db_schema::models::pool::members::MemberForm;
use db_schema::{PoolId};
use db_views::models::pool::members_views::MemberView;
use errors_lib_rs::model::{ApiError,};

#[async_trait::async_trait(?Send)]
impl ManagementTrait<MemberRequest> for MemberApi {
    type Response = HttpResponse;
    async fn create_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<MemberRequest>,
    ) -> Result<self::HttpResponse, ApiError> {
        let _conn = connection(&_req);

        let _param = _req.match_info().get("pool_id").unwrap();

        let _pool_id = PoolId {
            0: String::from(_param),
        };

        let mut _member_form = vec![];
        for element in _data.users.clone() {
            let _form = MemberForm {
                pool_id: Some(_pool_id.clone()),
                user_id: Some(element),
                user_role: _data.user_role.clone(),
                created_by: String::from("created_by"),
                updated_by: String::from("updated_by"),
            };
            _member_form.push(_form);
        }

        MemberView::create_item(&_conn, &_member_form, &_single.fields, &_single.expand).await?;

        Ok(HttpResponse::Ok().finish())
    }

    async fn delete_collection(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<MemberRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);

        let _header_pool_id = &_req.match_info().get("pool_id").unwrap();

        let _pool_id = PoolId(_header_pool_id.to_string());

        let _response = MemberView::delete_items(&_conn, &_pool_id, &_data.users).await?;

        Ok(HttpResponse::NoContent().finish())
    }

    async fn delete_item(
        _req: HttpRequest,
        _single: SingularRequest,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);
        let _header_pool_id = &_req.match_info().get("pool_id").unwrap();

        let _pool_id = PoolId(_header_pool_id.to_string());

        let _header_user_id = &_req.match_info().get("user_id").unwrap();

        let _user_id = _header_user_id.to_string();

        let _member_id = member::get_item_by_member_id(&_conn, &_pool_id, &_user_id)?;

        let __response = MemberView::delete_item(&_conn, &_member_id).await?;
        Ok(HttpResponse::NoContent().finish())
    }
    async fn get_item(
        _req: HttpRequest,
        _single: SingularRequest,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);

        let _header_pool_id = &_req.match_info().get("pool_id").unwrap();

        let _pool_id = PoolId(_header_pool_id.to_string());

        let _header_user_id = &_req.match_info().get("user_id").unwrap();

        let _user_id = _header_user_id.to_string();

        let _member_id = member::get_item_by_member_id(&_conn, &_pool_id, &_user_id)?;

        let _response =
            MemberView::get_item(&_conn, &_member_id, &_single.fields, &_single.expand).await?;

        let _body = serde_json::to_string(&_response)?;
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(_body))
    }

    async fn get_collection(
        _req: HttpRequest,
        _coll: CollectionRequest,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);

        let _param = _req.match_info().get("pool_id").unwrap();

        let mut _q = String::from("pool_id = '");

        _q.push_str(_param);

        _q.push_str("'");

        if let Some(val) = &_coll.q {
            _q.push_str(" and ");
            _q.push_str(val);
        }

        let _response = MemberView::get_collection(
            &_conn,
            &_coll.fields,
            &_coll.expand,
            &Some(_q),
            &_coll.sort,
            &_coll.offset,
            &_coll.limit,
            &_coll.total_count,
        )
        .await?;

        let body = serde_json::to_string(&_response)?;

        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body))
    }
}