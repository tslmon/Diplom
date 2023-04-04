// use crate::{Crud, ManagementAsyncTrait, PgConnection, option_value};
// use db_schema::models::job::events::{EventAggregation, Organizer};
// use db_schema::models::job::job_candidates::Candidate;
// use db_schema::schema::jobs::{self};
// use db_schema::{
//     models::job::{
//         events::{Event, EventData, EventForm},
//         jobs::Job,
//         stages::Stage,
//     },
//     schema::{
//         job_events::{self, dsl::*},
//         job_stages::{self},
//     },
//     EventId, RecruitError,
// };
// use diesel::{
//     dsl::{count_star, sql},
//     insert_into, ExpressionMethods, QueryDsl, RunQueryDsl,
// };
// use errors_lib_rs::{db::ModelErrorMessage, model::ModelError};
// impl Crud<EventForm, EventId> for Event {
//     fn create(_conn: &PgConnection, _form: &EventForm) -> Result<Self, ModelError> {
//         let _result = insert_into(job_events)
//             .values(_form)
//             .get_result::<Self>(_conn);
//         match _result {
//             Ok(res) => Ok(res),
//             Err(_err) => Err(RecruitError::diesel_error(_err)),
//         }
//     }

//     fn read(_conn: &PgConnection, _student: &EventId) -> Result<Self, ModelError> {
//         let _result = job_events
//             .filter(job_events::id.eq(_student.clone()))
//             .first::<Self>(_conn);
//         match _result {
//             Ok(res) => Ok(res),
//             Err(_err) => Err(RecruitError::diesel_error(_err)),
//         }
//     }

//     fn update(
//         _conn: &PgConnection,
//         _event: &EventId,
//         _form: &EventForm,
//     ) -> Result<Self, ModelError> {
//         let _result = diesel::update(job_events.filter(job_events::id.eq(_event.clone())))
//             .set(_form)
//             .get_result::<Self>(_conn);
//         match _result {
//             Ok(res) => Ok(res),
//             Err(_err) => Err(RecruitError::diesel_error(_err)),
//         }
//     }

//     fn update_only(
//         _conn: &PgConnection,
//         _event: &EventId,
//         _form: &EventForm,
//         _columns: Vec<String>,
//     ) -> Result<Self, ModelError> {
//         let _edited_student = _form.clone();
//         Self::update(&_conn, _event, &_edited_student)
//     }

//     fn delete(_conn: &PgConnection, _event: &EventId) -> Result<usize, ModelError> {
//         let _result =
//             diesel::delete(job_events.filter(job_events::id.eq(_event.clone()))).execute(_conn);
//         match _result {
//             Ok(res) => Ok(res),
//             Err(_err) => Err(RecruitError::diesel_error(_err)),
//         }
//     }
// }

// #[async_trait::async_trait(?Send)]
// impl ManagementAsyncTrait<EventForm, EventId> for Event {
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
//             (None, None) => job_events.into_boxed(),
//         };
//         if let Some(total_count) = _total_count {
//             if *total_count {
//                 let _total_sql = match _q {
//                     Some(q) => Self::event_by_filter(q),
//                     None => job_events.into_boxed(),
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
//         let mut _result = query.load::<Event>(_conn);
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
//         _event: &EventId,
//         _fields: &Option<Vec<String>>,
//         _expand: &Option<Vec<String>>,
//     ) -> Result<Self, ModelError>
//     where
//         Self: Sized,
//     {
//         Event::read(_conn, _event)
//     }
//     async fn create_item(
//         _conn: &PgConnection,
//         _form: &EventForm,
//         _fields: &Option<Vec<String>>,
//         _expand: &Option<Vec<String>>,
//     ) -> Result<Self, ModelError>
//     where
//         Self: Sized,
//     {
//         Event::create(_conn, _form)
//     }
//     async fn update_item(
//         _conn: &PgConnection,
//         _event: &EventId,
//         _form: &EventForm,
//         _fields: &Option<Vec<String>>,
//         _expand: &Option<Vec<String>>,
//     ) -> Result<Self, ModelError>
//     where
//         Self: Sized,
//     {
//         Event::update(_conn, _event, _form)
//     }
//     async fn delete_item(_conn: &PgConnection, _event: &EventId) -> Result<usize, ModelError> {
//         Event::delete(_conn, _event)
//     }
// }
// #[async_trait::async_trait(?Send)]
// pub trait Event_ {
//     /// Helper functions
//     fn event_by_filter<'a>(_sql: &'a str) -> job_events::BoxedQuery<'a, diesel::pg::Pg> {
//         job_events::table.filter(sql(_sql)).into_boxed()
//     }
//     async fn create_items(
//         _conn: &PgConnection,
//         _form: &Vec<EventForm>,
//         _fields: &Option<Vec<String>>,
//         _expand: &Option<Vec<String>>,
//     ) -> Result<Vec<Self>, ModelError>
//     where
//         Self: Sized;

//     async fn get_items(
//         _conn: &PgConnection,
//         _event: &Vec<EventId>,
//         _offset: &Option<i64>,
//         _limit: &Option<i64>,
//         _expand: &(bool, bool)
//     ) -> Result<Vec<EventData>, ModelError>
//     where
//         EventData: Sized,
//     {
//         let _result = job_events::table
//             .inner_join(jobs::table)
//             .left_join(job_stages::table)
//             .filter(job_events::id.eq_any(_event))
//             .limit(_limit.unwrap_or(10))
//             .offset(_offset.unwrap_or(0))
//             .load::<(Event, Job, Option<Stage>)>(_conn);
//         match _result {
//             Ok(items) => {
//                 let mut _event_data = vec![];
//                 for element in items {
//                     let _item = EventData {
//                         id: element.0.id,
//                         job_id: element.0.job_id,
//                         job_data: element.1,
//                         stage_id: element.0.stage_id,
//                         stage_data: option_value(element.2, _expand.0),
//                         type_: element.0.type_,
//                         name: element.0.name,
//                         description: element.0.description,
//                         sequence: element.0.sequence,
//                         metadata: element.0.metadata,
//                         app_metadata: element.0.app_metadata,
//                         created_by: element.0.created_by,
//                         created_at: element.0.created_at,
//                         updated_by: element.0.updated_by,
//                         updated_at: element.0.updated_at,
//                     };
//                     _event_data.push(_item);
//                 }

//                 Ok(_event_data)
//             }
//             Err(_err) => return Err(RecruitError::diesel_error(_err)),
//         }
//     }
// }

// #[async_trait::async_trait(?Send)]
// impl Event_ for Event {
//     async fn create_items(
//         _conn: &PgConnection,
//         _form: &Vec<EventForm>,
//         _fields: &Option<Vec<String>>,
//         _expand: &Option<Vec<String>>,
//     ) -> Result<Vec<Self>, ModelError>
//     where
//         Self: Sized,
//     {
//         let _result = insert_into(job_events)
//             .values(_form)
//             .get_results::<Self>(_conn);
//         match _result {
//             Ok(res) => Ok(res),
//             Err(_err) => return Err(RecruitError::diesel_error(_err)),
//         }
//     }
// }

// fn get_event_aggregations(
//     _conn: &PgConnection,
//     _events: &Vec<Event>,
//     _offset: &Option<i64>,
//     _limit: &Option<i64>,
//     _fields: &Option<Vec<String>>,
//     _expand: &bool,
// ) -> Result<Option<Vec<Vec<EventAggregation>>>, ModelError>
// where
//     Self: Sized,
// {
//     if *_expand {
//         let _result = EventAggregation::belonging_to(_events)
//             .limit(_limit.unwrap_or(10))
//             .load::<EventAggregation>(_conn);
//         match _result {
//             Ok(_aggregations) => {
//                 let _grouped_aggregations: Vec<Vec<EventAggregation>> =
//                     _aggregations.grouped_by(_events);
//                 Ok(Some(_grouped_aggregations))
//             }
//             Err(_err) => return Err(RecruitError::diesel_error(_err)),
//         }
//     } else {
//         Ok(None)
//     }
// }
// async fn get_event_candidates(
//     _conn: &PgConnection,
//     _events: &Vec<Event>,
//     _offset: &Option<i64>,
//     _limit: &Option<i64>,
//     _fields: &Option<Vec<String>>,
//     _expand: &bool,
// ) -> Result<Option<Vec<Vec<Candidate>>>, ModelError>
// where
//     Self: Sized,
// {
//     if *_expand {
//         let _result = Candidate::belonging_to(_events).load::<Candidate>(_conn);

//         match _result {
//             Ok(topics) => {
//                 let _grouped_topics: Vec<Vec<Candidate>> = topics.grouped_by(_events);
//                 Ok(Some(_grouped_topics))
//             }
//             Err(_err) => return Err(RecruitError::diesel_error(_err)),
//         }
//     } else {
//         Ok(None)
//     }
// }
// async fn get_event_organizers(
//     _conn: &PgConnection,
//     _events: &Vec<Event>,
//     _offset: &Option<i64>,
//     _limit: &Option<i64>,
//     _fields: &Option<Vec<String>>,
//     _expand: &bool,
// ) -> Result<Option<Vec<Vec<Event>>>, ModelError>
// where
//     Self: Sized,
// {
//     if *_expand {
//         let _result = Event::belonging_to(_events).load::<Event>(_conn);

//         match _result {
//             Ok(topics) => {
//                 let _grouped_topics: Vec<Vec<Event>> = topics.grouped_by(_events);
//                 Ok(Some(_grouped_topics))
//             }
//             Err(_err) => return Err(RecruitError::diesel_error(_err)),
//         }
//     } else {
//         Ok(None)
//     }
// }

// async fn get_event_childs(
//     _conn: &PgConnection,
//     _events: &Vec<Event>,
//     _fields: &Option<Vec<String>>,
//     _expand: &Option<Vec<String>>,
// ) -> Result<
//     (
//         Option<Vec<Vec<EventAggregation>>>,
//         Option<Vec<Vec<Candidate>>>,
//         Option<Vec<Vec<Organizer>>>,
//     ),
//     ModelError,
// > {
//     let _return = match _expand {
//         Some(val) => {
//             let exec_plan: (bool, bool, bool) =
//                 if val.iter().any(|a| a == "all") {
//                     (true, true, true)
//                 } else {
//                     (
//                         val.iter().any(|a| a == "aggregations"),
//                         val.iter().any(|a| a == "candidates"),
//                         val.iter().any(|a| a == "organizer"),
//                     )
//                 };
//             let aggregations_fut = Event::get_event_aggregations(
//                 _conn,
//                 _events,
//                 &None,
//                 &None,
//                 &None,
//                 &exec_plan.0,
//             );
//             let candidate_fut = Event::get_event_candidates(
//                 _conn,
//                 _events,
//                 &None,
//                 &None,
//                 &None,
//                 &exec_plan.1,
//             );
//             let event_fut = Event::get_event_organizer(
//                 _conn,
//                 _events,
//                 &None,
//                 &None,
//                 &None,
//                 &exec_plan.2,
//             );

//             try_join!(
//                 aggregations_fut,
//                 candidate_fut,
//                 organizer_fut,
//             )?
//         }
//         None => (None, None, None),
//     };
//     Ok(_return)
// }

// #[async_trait::async_trait(?Send)]
// impl ManagementAsyncTrait<EventForm, EventId> for EventData {}
