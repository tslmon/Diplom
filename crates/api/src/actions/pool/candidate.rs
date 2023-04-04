use crate::actions::job::ManagementTrait;
use crate::common::pool::candidates::{
    CandidateApi, CandidateMoveApi, CandidateMoveRequest, CandidateRequest, CandidateDuplicateRequest, CandidateDuplicateApi,
};
use crate::common::{connection, CollectionRequest, SingularRequest};
use actix_web::http::header::ContentType;
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse};
use db_queries::Crud;
use db_schema::models::pool::candidates::{CandidateForm, Candidate};
use db_schema::{CandidateId, PoolId};
use db_views::models::pool::candidates_views::CandidateView;
use errors_lib_rs::model::{ApiError, ApiErrorEnum};
#[async_trait::async_trait(?Send)]
impl ManagementTrait<CandidateRequest> for CandidateApi {
    type Response = HttpResponse;
    async fn create_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<CandidateRequest>,
    ) -> Result<self::HttpResponse, ApiError> {
        let _conn = connection(&_req);
        let _header_pool_id = &_req.match_info().get("pool_id").unwrap();
        let _pool_id = PoolId(_header_pool_id.to_string());
        let val = String::from("Zion Mongolia");
        let val1 = String::from("Zion Mongolia");
        let _form = CandidateForm {
            pool_id: Some(_pool_id),
            user_id: _data.user_id.clone(),
            profile_id: _data.profile_id.clone(),
            name: _data.name.clone(),
            description: _data.description.clone(),
            originating_candidate_id: _data.originating_candidate_id.clone(),
            metadata: _data.metadata.clone(),
            app_metadata: _data.app_metadata.clone(),
            created_by: Some(val),
            updated_by: Some(val1),
        };
        let _response =
            CandidateView::create_item(&_conn, &_form, &_single.fields, &_single.expand).await?;
        let _body = serde_json::to_string(&_response)?;
        Ok(HttpResponse::Created()
            .content_type(ContentType::json())
            .body(_body))
    }
    async fn update_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<CandidateRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);
        let _method = _req.method();
        let aaaa = (
            
        _data.name.clone(), 
        _data.description.clone(),
        _data.metadata.clone(),
        _data.app_metadata.clone());

        if "PUT" == _method.as_str() {
            if aaaa.0.is_some()
             && aaaa.1.is_some() 
             && aaaa.2.is_some()
             && aaaa.3.is_some(){
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
        let _path_candidate_id = &_req.match_info().get("candidate_id").unwrap();

        let _candidate_id = CandidateId(_path_candidate_id.to_string());
        let val = String::from("Zion Mongolia");
        let val1 = String::from("Zion Mongolia");
        let _form = CandidateForm {
            pool_id: None,
            user_id: _data.user_id.clone(),
            profile_id: _data.profile_id.clone(),
            name: _data.name.clone(),
            description: _data.description.clone(),
            originating_candidate_id: _data.originating_candidate_id.clone(),
            metadata: _data.metadata.clone(),
            app_metadata: _data.metadata.clone(),
            created_by: Some(val),
            updated_by: Some(val1),
        };
        CandidateView::update_item(
            &_conn,
            &_candidate_id,
            &_form,
            &_single.fields,
            &_single.expand,
        )
        .await?;
        let _response =
            CandidateView::get_item(&_conn, &_candidate_id, &_single.fields, &_single.expand)
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

        let _header_candidate_id = &_req.match_info().get("candidate_id").unwrap();

        let _candidate_id = CandidateId(_header_candidate_id.to_string());
        let f =
            CandidateView::delete_item(&_conn, &_candidate_id, &_single.fields, &_single.expand)
                .await?;

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

        let _path_candidate_id = &_req.match_info().get("candidate_id").unwrap();

        let _candidate_id = CandidateId(_path_candidate_id.to_string());

        let _response =
            CandidateView::get_item(&_conn, &_candidate_id, &_single.fields, &_single.expand)
                .await?;
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

        let _response = CandidateView::get_collection(
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
            .reason("Candidate successfully retrived.")
            .body(body))
    }
}

#[async_trait::async_trait(?Send)]
impl ManagementTrait<CandidateMoveRequest> for CandidateMoveApi {
    type Response = HttpResponse;
    async fn update_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<CandidateMoveRequest>,
    ) -> Result<HttpResponse, ApiError> {
        let _conn = connection(&_req);
        let _path_pool_id = &_req.match_info().get("pool_id").unwrap();
        let _pool_id = PoolId(_path_pool_id.to_string());
        let _path_candidate_id = &_req.match_info().get("candidate_id").unwrap();
        let _candidate_id = CandidateId(_path_candidate_id.to_string());
        let _form = CandidateForm {
            pool_id: _data.pool_id.clone(),
            user_id: None,
            profile_id: None,
            name: None,
            description: None,
            originating_candidate_id: None,
            metadata:None,
            app_metadata: None,
            created_by: None,
            updated_by: None,
        };
        let _response = CandidateView::update_item(
            &_conn,
            &_candidate_id,
            &_form,
            &_single.fields,
            &_single.expand,
        )
        .await?;
        let _body = serde_json::to_string(&_response)?;

        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(_body))
    }
}

#[async_trait::async_trait(?Send)]
impl ManagementTrait<CandidateDuplicateRequest> for CandidateDuplicateApi {
    type Response = HttpResponse;
    async fn update_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<CandidateDuplicateRequest>,
    ) -> Result<HttpResponse, ApiError> {
        let _conn = connection(&_req);

        let _path_pool_id = &_req.match_info().get("pool_id").unwrap();

        let _pool_id = PoolId(_path_pool_id.to_string());

        let _path_candidate_id = &_req.match_info().get("candidate_id").unwrap();

        let _candidate_id = CandidateId(_path_candidate_id.to_string());
    
        let _response =
        Candidate::read(&_conn, &_candidate_id)?;
        let _form = CandidateForm {
            pool_id: _data.pool_id.clone(), 
            user_id: Some(_response.user_id),
            profile_id: Some(_response.profile_id),
            name: Some(_response.name),
            description: _response.description,
            originating_candidate_id: Some(_response.originating_candidate_id),
            metadata: Some(_response.metadata),
            app_metadata: Some(_response.app_metadata),
            created_by: Some(_response.created_by),
            updated_by:Some(_response.updated_by),
        };

        let _response = CandidateView::create_item(
            &_conn,
            &_form,
            &_single.fields,
            &_single.expand,
        )
        .await?;
        let _body = serde_json::to_string(&_response)?;

        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(_body))
    }
}
