use crate::actions::job::ManagementTrait;
use crate::common::pool::pools::{PoolApi, PoolRequest};
use crate::common::{connection, CollectionRequest, SingularRequest};
use actix_web::http::header::ContentType;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse};
use db_schema::models::pool::pools::PoolForm;
use db_schema::PoolId;
use db_views::models::pool::pools_views::PoolView;
use errors_lib_rs::model::{ApiError, ApiErrorEnum};
#[async_trait::async_trait(?Send)]
impl ManagementTrait<PoolRequest> for PoolApi {
    type Response = HttpResponse;
    async fn create_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<PoolRequest>,
    ) -> Result<self::HttpResponse, ApiError> {
        let _conn = connection(&_req);
        let val=String::from("Zion Mongolia");
        let val1=String::from("Zion Mongolia");
            let _form = PoolForm {
                name: _data.name.clone(),
                description: _data.description.clone(),
                metadata: _data.metadata.clone(),
                app_metadata: _data.app_metadata.clone(),
                created_by: Some(val),
                updated_by: Some(val1),
            };
            let _pool_id =
            PoolView::create_item(&_conn, &_form, &_single.fields, &_single.expand).await?;
            // println!("{:#?}",_pool_id);
            let _response =
                PoolView::get_item(&_conn, &_pool_id, &_single.fields, &_single.expand).await?;
            let _body = serde_json::to_string(&_response)?;
            // println!("{:#?}",_body);
            Ok(HttpResponse::Created()
                .content_type(ContentType::json())
                .body(_body))

    }
    async fn update_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<PoolRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);
        let _method = _req.method();

        let aaaa = (
            _data.name.clone(),
            _data.description.clone(),
            _data.metadata.clone(),
            _data.app_metadata.clone(),
        );

        if "PUT" == _method.as_str() {
            if aaaa.0.is_some()
                && aaaa.1.is_some()
            {
            } else {
                let _err = ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    "All columns required in PUT action. See https://developer.hischool.one/api/"
                        .to_string(),
                );

                return Err(_err);
            }
        }
        let _path_pool_id = &_req.match_info().get("pool_id").unwrap();
        let val=String::from("Zion Mongolia");
        let val1=String::from("Zion Mongolia");
        let _pool_id = PoolId(_path_pool_id.to_string());
        let _form = PoolForm {
            name: _data.name.clone(),
            description: _data.description.clone(),
            metadata: _data.metadata.clone(),
            app_metadata: _data.app_metadata.clone(),
            created_by: Some(val),
            updated_by: Some(val1),
        };
        PoolView::update_item(
            &_conn,
            &_pool_id,
            &_form,
            &_single.fields,
            &_single.expand,
        )
        .await?;
    let _response =
    PoolView::get_item(&_conn, &_pool_id, &_single.fields, &_single.expand)
        .await?;
        let _body = serde_json::to_string(&_response)?;
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(_body))
    }
    async fn delete_item(
        _req: HttpRequest,
        _single: SingularRequest,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);

        let _header_pool_id = &_req.match_info().get("pool_id").unwrap();

        let _pool_id = PoolId(_header_pool_id.to_string());
        let f =
            PoolView::delete_item(&_conn, &_pool_id, &_single.fields, &_single.expand).await?;

        if f == 1 {
            Ok(HttpResponse::NoContent().finish())
        } else {
            Ok(HttpResponse::NotFound()
                .content_type(ContentType::plaintext())
                .body("NotFound"))
        }
    }
    async fn get_item(
        _req: HttpRequest,
        _single: SingularRequest,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);

        let _path_pool_id = &_req.match_info().get("pool_id").unwrap();

        let _pool_id = PoolId(_path_pool_id.to_string());

        let _response =
            PoolView::get_item(&_conn, &_pool_id, &_single.fields, &_single.expand).await?;
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

        let _response = PoolView::get_collection(
            &_conn,
            &_coll.fields,
            &_coll.expand,
            &_coll.q,
            &_coll.sort,
            &_coll.offset,
            &_coll.limit,
            &_coll.total_count,
        )
        .await?;
        let body = serde_json::to_string(&_response)?;
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .reason("Pool successfully retrived.")
            .body(body))
    }
}
