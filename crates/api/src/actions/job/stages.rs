use actix_http::body::BoxBody;
use actix_web::{HttpResponse, HttpRequest, web::Json, http::header::ContentType, ResponseError};
use db_schema::{models::job::stages::StageForm, StageId, JobId};
use db_views::models::job::stages_view::StagesView;
use errors_lib_rs::model::{ApiError, ModelError, ApiErrorEnum};

use crate::common::{job::stages::{StagesRequest, StagesApi, StagesStatusRequest, StagesStatusApi}, SingularRequest, connection, CollectionRequest};

use super::ManagementTrait;
#[async_trait::async_trait(?Send)]
impl ManagementTrait<StagesRequest> for StagesApi {
    type Response = HttpResponse;
    async fn create_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<StagesRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);

        let _header_job_id = &_req.match_info().get("job_id").unwrap();

        let _job_id = JobId(_header_job_id.to_string());

        if _data.name.is_some() {
            let form = StageForm { 
                job_id:  Some(_job_id),
                type_: _data.type_.clone(),
                name: _data.name.clone(),
                description: _data.description.clone(),
                sequence: _data.sequence.clone(),
                metadata: _data.metadata.clone(),
                app_metadata: _data.metadata.clone(),
                created_by: Some(String::from("me")),
                updated_by: Some(String::from("u")),
            };

            let _response =
                StagesView::create_item(&_conn, &form, &_single.fields, &_single.expand).await?;
            let _body = serde_json::to_string(&_data)?;

            Ok(HttpResponse::Created()
                .content_type(ContentType::json())
                .body(_body))
        } else {
            let _err = ApiError::from(ModelError::QuerySyntax(Some(
                "A code or name must be required.".to_string(),
            )));
            let _body = BoxBody::new(serde_json::to_string(&_err.error_body)?);
            Ok(HttpResponse::build(_err.status_code()).json(_err.error_body.clone()))
        }
    }
    async fn update_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<StagesRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);

        let _method = _req.method();

        let aaaa = (
            _data.name.clone(),
            _data.description.clone(),
           _data.sequence.clone(),
            _data.metadata.clone(),
            _data.app_metadata.clone(),
        );

        if "PUT" == _method.as_str() {
            if aaaa.0.is_some()
                && aaaa.1.is_some()
                && aaaa.2.is_some()
                && aaaa.3.is_some()
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

        let _path_stages_id = &_req.match_info().get("stage_id").unwrap();

        let _stages_id = StageId(_path_stages_id.to_string());

            let form = StageForm {
            job_id:  None,
            type_: _data.type_.clone(),
            name: _data.name.clone(),
            description: _data.description.clone(),
            sequence: _data.sequence.clone(),
            metadata: _data.metadata.clone(),
            app_metadata: _data.app_metadata.clone(),
            created_by: None,
            updated_by: None,
        };

        let _response =
            StagesView::update_item(&_conn, &_stages_id, &form, &_single.fields, &_single.expand)
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

        let _header_class_id = &_req.match_info().get("stage_id").unwrap();

        let _class_id = StageId(_header_class_id.to_string());

        let f = StagesView::delete_item(&_conn, &_class_id, &_single.fields, &_single.expand)
            .await?;

        if f == 1 {
            Ok(HttpResponse::NoContent()
                .reason("Classroom successfully deleted.")
                .finish())
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

        let _path_class_id = &_req.match_info().get("stage_id").unwrap();

        let _class_id = StageId(_path_class_id.to_string());

        let _response =
            StagesView::get_item(&_conn, &_class_id, &_single.fields, &_single.expand).await?;

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

        let _response = StagesView::get_collection(
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
            .reason("Tenant successfully retrived.")
            .body(body))
    }
}
#[async_trait::async_trait(?Send)]
impl ManagementTrait<StagesStatusRequest> for StagesStatusApi {
    type Response = HttpResponse;
    async fn update_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<StagesStatusRequest>,
    ) -> Result<HttpResponse, ApiError> {
        let _conn = connection(&_req);

        let _path_stages_id = &_req.match_info().get("stage_id").unwrap();

        let _stages_id = StageId(_path_stages_id.to_string());

        let _form = StageForm {
            job_id: None,
            name: None,
            type_: None,
            description: None,
            sequence: None,
            metadata: None,
            app_metadata: None,
            created_by: None,
            updated_by: None,
        };

        let _response = StagesView::update_item(
            &_conn,
            &_stages_id,
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
