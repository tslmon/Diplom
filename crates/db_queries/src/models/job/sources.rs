// use db_schema::{schema::job_stages::{self, dsl::*}, models::job::{stages::StageData, jobs::Job}};
// use crate::{Crud, ManagementAsyncTrait, PgConnection};
// use db_schema::{schema::{jobs::{self}}, StageId, models::job::stages::Stage, RecruitError};

// use diesel::{
//     dsl::{count_star, sql},
//     insert_into, ExpressionMethods, QueryDsl, RunQueryDsl,
// };

// use db_schema::models::job::stages::StageForm;
// use errors_lib_rs::{db::ModelErrorMessage, model::ModelError};
// impl Crud<StageForm, StageId> for Stage {
//     fn create(_conn: &PgConnection, _form: &StageForm) -> Result<Self, ModelError> {
//         let _result = insert_into(job_stages)
//             .values(_form)
//             .get_result::<Self>(_conn);
//         match _result {
//             Ok(res) => Ok(res),
//             Err(_err) => Err(RecruitError::diesel_error(_err)),
//         }
//     }

//     fn read(_conn: &PgConnection, _student: &StageId) -> Result<Self, ModelError> {
//         let _result = job_stages
//             .filter(job_stages::id.eq(_student.clone()))
//             .first::<Self>(_conn);
//         match _result {
//             Ok(res) => Ok(res),
//             Err(_err) => Err(RecruitError::diesel_error(_err)),
//         }
//     }

//     fn update(
//         _conn: &PgConnection,
//         _event: &StageId,
//         _form: &StageForm,
//     ) -> Result<Self, ModelError> {
//         let _result = diesel::update(job_stages.filter(job_stages::id.eq(_event.clone())))
//             .set(_form)
//             .get_result::<Self>(_conn);
//         match _result {
//             Ok(res) => Ok(res),
//             Err(_err) => Err(RecruitError::diesel_error(_err)),
//         }
//     }

//     fn update_only(
//         _conn: &PgConnection,
//         _event: &StageId,
//         _form: &StageForm,
//         _columns: Vec<String>,
//     ) -> Result<Self, ModelError> {
//         let _edited_student = _form.clone();
//         Self::update(&_conn, _event, &_edited_student)
//     }

//     fn delete(_conn: &PgConnection, _event: &StageId) -> Result<usize, ModelError> {
//         let _result =
//             diesel::delete(job_stages.filter(job_stages::id.eq(_event.clone()))).execute(_conn);
//         match _result {
//             Ok(res) => Ok(res),
//             Err(_err) => Err(RecruitError::diesel_error(_err)),
//         }
//     }
// }

// #[async_trait::async_trait(?Send)]
// impl ManagementAsyncTrait<StageForm, StageId> for Stage {
//     async fn get_collection(
//         _conn: &PgConnection,
//         _fields: &Option<Vec<String>>,
//         _expand: &Option<Vec<String>>,
//         _q: &Option<String>,
//         _sort: &Option<Vec<String>>,
//         _offset: &Option<i64>,
//         _limit: &Option<i64>,
//         _total_count: &Option<bool>,
//     ) -> Result<(Vec<Self>, Option<i64>, bool), ModelError>
//     where
//         Self: Sized,
//     {
//         let mut l_return_count: Option<i64> = None;
//         let mut s = String::new();
//         let mut query = match (_q, _sort) {
//             (Some(q), Some(sort)) => {
//                 s.push_str(q);
//                 s.push_str(" ORDER BY ");
//                 s.push_str(&sort.join(", "));
//                 Self::event_by_filter(&s)
//             }
//             (Some(q), None) => Self::event_by_filter(q),
//             (None, Some(sort)) => {
//                 s.push_str(" 1=1 ORDER BY ");
//                 s.push_str(&sort.join(", "));
//                 Self::event_by_filter(&s)
//             }
//             (None, None) => job_stages.into_boxed(),
//         };
//         if let Some(total_count) = _total_count {
//             if *total_count {
//                 let _total_sql = match _q {
//                     Some(q) => Self::event_by_filter(q),
//                     None => job_stages.into_boxed(),
//                 };
//                 let _result = _total_sql.select(count_star()).first::<i64>(_conn);
//                 match _result {
//                     Ok(res) => l_return_count = Some(res),
//                     Err(_err) => return Err(RecruitError::diesel_error(_err)),
//                 }
//             }
//         }
//         if let Some(offset) = _offset {
//             query = query.offset(*offset);
//         }
//         if let Some(limit) = _limit {
//             query = query.limit(*limit + 1);
//         }
//         let mut _result = query.load::<Stage>(_conn);
//         let mut _data = match _result {
//             Ok(res) => res,
//             Err(_err) => return Err(RecruitError::diesel_error(_err)),
//         };
//         let l_has_more = _data.len() > _limit.unwrap() as usize;
//         let mut _l_limit: usize;
//         if l_has_more {
//             _l_limit = _limit.unwrap() as usize;
//             _data = (_data[.._l_limit]).to_vec();
//         }
//         Ok((_data, l_return_count, l_has_more))
//     }
//     async fn get_item(
//         _conn: &PgConnection,
//         _event: &StageId,
//         _fields: &Option<Vec<String>>,
//         _expand: &Option<Vec<String>>,
//     ) -> Result<Self, ModelError>
//     where
//         Self: Sized,
//     {
//         Stage::read(_conn, _event)
//     }
//     async fn create_item(
//         _conn: &PgConnection,
//         _form: &StageForm,
//         _fields: &Option<Vec<String>>,
//         _expand: &Option<Vec<String>>,
//     ) -> Result<Self, ModelError>
//     where
//         Self: Sized,
//     {
//         Stage::create(_conn, _form)
//     }
//     async fn update_item(
//         _conn: &PgConnection,
//         _event: &StageId,
//         _form: &StageForm,
//         _fields: &Option<Vec<String>>,
//         _expand: &Option<Vec<String>>,
//     ) -> Result<Self, ModelError>
//     where
//         Self: Sized,
//     {
//         Stage::update(_conn, _event, _form)
//     }
//     async fn delete_item(_conn: &PgConnection, _event: &StageId) -> Result<usize, ModelError> {
//         Stage::delete(_conn, _event)
//     }
// }
// #[async_trait::async_trait(?Send)]
// pub trait Stage_ {
//     /// Helper functions
//     fn event_by_filter<'a>(_sql: &'a str) -> job_stages::BoxedQuery<'a, diesel::pg::Pg> {
//         job_stages::table.filter(sql(_sql)).into_boxed()
//     }
//     async fn create_items(
//         _conn: &PgConnection,
//         _form: &Vec<StageForm>,
//     ) -> Result<Vec<Self>, ModelError>
//     where
//         Self: Sized;

//     async fn get_items(
//         _conn: &PgConnection,
//         _event: &Vec<StageId>,
//         _offset: &Option<i64>,
//         _limit: &Option<i64>,
//         _expand: &(bool, bool)
//     ) -> Result<Vec<StageData>, ModelError>
//     where
//     StageData: Sized,
//     {
//         let _result = job_stages::table
//             .inner_join(jobs::table)
//             .filter(job_stages::id.eq_any(_event))
//             .limit(_limit.unwrap_or(10))
//             .offset(_offset.unwrap_or(0))
//             .load::<(Stage, Job)>(_conn);
//         match _result {
//             Ok(items) => {
//                 let mut _student_data = vec![];
//                 for element in items {
//                     let _item = StageData {
//                         id: element.0.id,
//                         job_id: element.0.job_id,
//                         job_data: element.1,
//                         type_: element.0.type_,
//                         name: element.0.name,
//                         description: element.0.description,
//                         sequence: element.0.sequence,
//                         locked: element.0.locked,
//                         metadata: element.0.metadata,
//                         app_metadata: element.0.app_metadata,
//                         created_by: element.0.created_by,
//                         created_at: element.0.created_at,
//                         updated_by: element.0.updated_by,
//                         updated_at: element.0.updated_at,
//                     };
//                     _student_data.push(_item);
//                 }

//                 Ok(_student_data)
//             }
//             Err(_err) => return Err(RecruitError::diesel_error(_err)),
//         }
//     }
// }

// #[async_trait::async_trait(?Send)]
// impl Stage_ for Stage {
//     async fn create_items(
//         _conn: &PgConnection,
//         _form: &Vec<StageForm>,
//     ) -> Result<Vec<Self>, ModelError>
//     where
//         Self: Sized,
//     {
//         let _result = insert_into(job_stages)
//             .values(_form)
//             .get_results::<Self>(_conn);
//         match _result {
//             Ok(res) => Ok(res),
//             Err(_err) => return Err(RecruitError::diesel_error(_err)),
//         }
//     }
// }

// #[async_trait::async_trait(?Send)]
// impl ManagementAsyncTrait<StageForm, StageId> for StageData {}
