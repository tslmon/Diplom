pub mod jobs;
pub mod stages;

use crate::common::{CollectionRequest, SingularRequest};
use actix_web::{
    web::{Json, Query},
    FromRequest, HttpRequest, Responder,
};
use errors_lib_rs::model::{ApiError, ApiErrorEnum};
use futures::future::{err, ok, Ready};
use std::collections::HashMap;
use utils::{QUERY_EXPAND_REGEX, QUERY_FIELDS_REGEX, QUERY_SORT_REGEX};

#[async_trait::async_trait(?Send)]
pub trait ManagementTrait<T: Sized + Send + 'static> {
    type Response: Responder;
    async fn create_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<T>,
    ) -> Result<Self::Response, ApiError> {
        todo!()
    }
    async fn update_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<T>,
    ) -> Result<Self::Response, ApiError> {
        todo!()
    }
    async fn delete_item(
        _req: HttpRequest,
        _single: SingularRequest,
    ) -> Result<Self::Response, ApiError> {
        todo!()
    }
    async fn delete_collection(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<T>,
    ) -> Result<Self::Response, ApiError> {
        todo!()
    }
    async fn get_item(
        _req: HttpRequest,
        _single: SingularRequest,
    ) -> Result<Self::Response, ApiError> {
        todo!()
    }
    async fn get_collection(
        _req: HttpRequest,
        _coll: CollectionRequest,
    ) -> Result<Self::Response, ApiError> {
        todo!()
    }
}

impl Default for SingularRequest {
    fn default() -> Self {
        Self {
            fields: None,
            expand: None,
        }
    }
}

impl SingularRequest {
    fn set_expand_from_str(&mut self, value: Option<&String>) -> Result<&Self, ApiError> {
        let mut _return_vec: Vec<String> = Vec::new();
        if let Some(_expand) = &value {
            if QUERY_EXPAND_REGEX.is_match(&_expand) {
                for mut item in _expand
                    .to_lowercase()
                    .split(',')
                    .into_iter()
                    .map(|a| a.split_whitespace())
                {
                    let mut _expand_str = String::new();
                    if let Some(val) = item.next() {
                        _expand_str.push_str(&val);
                    } else {
                        return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            String::from("Expand parameter included empty value."),
                        ));
                    };
                    if let Some(_) = item.next() {
                        return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            "Expand parameter ".to_owned() + &_expand_str + " was wrong.",
                        ));
                    };
                    if !_return_vec.iter().any(|a| a == &_expand_str) {
                        _return_vec.push(_expand_str);
                    }
                }
            } else {
                return Err(ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    String::from(
                        "Expand parameter's syntax was wrong. Please use (a-z, A-Z, '_', ',').",
                    ),
                ));
            }
            self.expand = Some(_return_vec);
        }
        Ok(self)
    }

    fn set_fields_from_str(&mut self, value: Option<&String>) -> Result<&Self, ApiError> {
        let mut _return_vec: Vec<String> = Vec::new();
        if let Some(_fields) = &value {
            if QUERY_FIELDS_REGEX.is_match(&_fields) {
                for mut item in _fields
                    .to_lowercase()
                    .split(',')
                    .into_iter()
                    .map(|a| a.split_whitespace())
                {
                    let mut _fields_str = String::new();
                    if let Some(val) = item.next() {
                        _fields_str.push_str(&val);
                    } else {
                        return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            String::from("Fields parameter included empty value."),
                        ));
                    };
                    if let Some(_) = item.next() {
                        return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            "Fields parameter ".to_owned() + &_fields_str + " was wrong.",
                        ));
                    };
                    if !_return_vec.iter().any(|a| a == &_fields_str) {
                        _return_vec.push(_fields_str);
                    }
                }
            } else {
                return Err(ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    String::from(
                        "Fields parameter's syntax was wrong. Please use (a-z, A-Z, '_', ',').",
                    ),
                ));
            }
            self.fields = Some(_return_vec);
        }
        Ok(self)
    }
}

impl FromRequest for SingularRequest {
    type Error = ApiError;
    type Future = Ready<Result<Self, ApiError>>;
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let query = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();
        let mut l_return = Self::default();
        match l_return.set_fields_from_str(query.get("fields")) {
            Ok(_) => (),
            Err(e) => return err(e),
        };
        match l_return.set_expand_from_str(query.get("expand")) {
            Ok(_) => (),
            Err(e) => return err(e),
        };
        ok(l_return)
    }
}
impl Default for CollectionRequest {
    fn default() -> Self {
        Self {
            fields: None,
            expand: None,
            q: None,
            total_count: None,
            sort: None,
            offset: Some(0),
            limit: Some(10),
        }
    }
}

impl CollectionRequest {
    fn set_sort_from_str(&mut self, value: Option<&String>) -> Result<&Self, ApiError> {
        let mut _return_vec: Vec<String> = Vec::new();
        if let Some(_sort) = &value {
            if QUERY_SORT_REGEX.is_match(&_sort) {
                for mut item in _sort
                    .to_lowercase()
                    .split(',')
                    .into_iter()
                    .map(|a| a.split_whitespace())
                {
                    let mut _sort_str = String::new();
                    if let Some(val) = item.next() {
                        _sort_str.push_str(&val);
                    } else {
                        return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            String::from("Sort parameter contains empty field."),
                        ));
                    };
                    if let Some(val) = item.next() {
                        if "asc".eq_ignore_ascii_case(val) || "desc".eq_ignore_ascii_case(val) {
                            _sort_str.push_str(" ");
                            _sort_str.push_str(&val);
                        } else {
                            return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            format!("Sort parameter must use 'asc' for ascending or 'desc' for descending. Wrong field name: '{}'", _sort_str),
                        ));
                        }
                    } else {
                        _sort_str.push_str(" asc");
                    };
                    if !_return_vec.iter().any(|a| a == &_sort_str) {
                        _return_vec.push(_sort_str);
                    }
                }
            } else {
                return Err(ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    String::from(
                        "Sort parameter's syntax was wrong. Please use (a-z, A-Z, '_', ',').",
                    ),
                ));
            }
            self.sort = Some(_return_vec);
        }
        Ok(self)
    }

    fn set_expand_from_str(&mut self, value: Option<&String>) -> Result<&Self, ApiError> {
        let mut _return_vec: Vec<String> = Vec::new();
        if let Some(_expand) = &value {
            if QUERY_EXPAND_REGEX.is_match(&_expand) {
                for mut item in _expand
                    .to_lowercase()
                    .split(',')
                    .into_iter()
                    .map(|a| a.split_whitespace())
                {
                    let mut _expand_str = String::new();
                    if let Some(val) = item.next() {
                        _expand_str.push_str(&val);
                    } else {
                        return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            String::from("Expand parameter included empty value."),
                        ));
                    };
                    if let Some(_) = item.next() {
                        return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            "Expand parameter ".to_owned() + &_expand_str + " was wrong.",
                        ));
                    };
                    if !_return_vec.iter().any(|a| a == &_expand_str) {
                        _return_vec.push(_expand_str);
                    }
                }
            } else {
                return Err(ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    String::from(
                        "Expand parameter's syntax was wrong. Please use (a-z, A-Z, '_', ',').",
                    ),
                ));
            }
            self.expand = Some(_return_vec);
        }
        Ok(self)
    }

    fn set_fields_from_str(&mut self, value: Option<&String>) -> Result<&Self, ApiError> {
        let mut _return_vec: Vec<String> = Vec::new();
        if let Some(_fields) = &value {
            if QUERY_FIELDS_REGEX.is_match(&_fields) {
                for mut item in _fields
                    .to_lowercase()
                    .split(',')
                    .into_iter()
                    .map(|a| a.split_whitespace())
                {
                    let mut _fields_str = String::new();
                    if let Some(val) = item.next() {
                        _fields_str.push_str(&val);
                    } else {
                        return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            String::from("Fields parameter included empty value."),
                        ));
                    };
                    if let Some(_) = item.next() {
                        return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            "Fields parameter ".to_owned() + &_fields_str + " was wrong.",
                        ));
                    };
                    if !_return_vec.iter().any(|a| a == &_fields_str) {
                        _return_vec.push(_fields_str);
                    }
                }
            } else {
                return Err(ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    String::from(
                        "Fields parameter's syntax was wrong. Please use (a-z, A-Z, '_', ',').",
                    ),
                ));
            }
            self.fields = Some(_return_vec);
        }
        Ok(self)
    }
}

impl FromRequest for CollectionRequest {
    type Error = ApiError;
    type Future = Ready<Result<Self, ApiError>>;
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let query = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();
        let mut l_return = Self::default();
        if let Some(l_q) = query.get("q") {
            if !l_q.is_empty() {
                l_return.q = Some(l_q.to_string());
            } else {
                return err(ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    String::from("Q parameter included empty value."),
                ));
            }
        };
        if let Some(l_total_count) = query.get("total_count") {
            l_return.total_count = Some("true".eq_ignore_ascii_case(l_total_count))
        };
        if let Some(l_offset) = query.get("offset") {
            let var = i64::from_str_radix(l_offset, 10);
            if var.is_err() {
                return err(ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    String::from("Offset parameter parse error. Invalid data type."),
                ));
            }
            l_return.offset = var.ok();
        };
        if let Some(l_limit) = query.get("limit") {
            let var = i64::from_str_radix(l_limit, 10);
            if var.is_err() {
                return err(ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    String::from("Limit parameter parse error. Invalid data type."),
                ));
            }
            l_return.limit = var.ok();
        };
        match l_return.set_sort_from_str(query.get("sort")) {
            Ok(_) => (),
            Err(e) => return err(e),
        };
        match l_return.set_expand_from_str(query.get("expand")) {
            Ok(_) => (),
            Err(e) => return err(e),
        };
        match l_return.set_fields_from_str(query.get("fields")) {
            Ok(_) => (),
            Err(e) => return err(e),
        };
        ok(l_return)
    }
}
