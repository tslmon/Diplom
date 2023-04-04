use db_schema::{schema::job_stages::{self, dsl::*}, models::job::{stages::{ StageAggregation, StageForm}, job_candidates::Candidate, events::Event}};
use crate::{Crud, ManagementAsyncTrait, PgConnection};
use db_schema::{StageId, models::job::stages::Stage, RecruitError};
use db_schema::models::job::stages::StageData;

use diesel::{
    dsl::{count_star, sql},
    insert_into, ExpressionMethods, QueryDsl, RunQueryDsl,
};
use crate::diesel::BelongingToDsl;
use errors_lib_rs::{db::ModelErrorMessage, model::ModelError};
use crate::diesel::GroupedBy;
use futures::try_join;
impl Crud<StageForm, StageId> for Stage {
    fn create(_conn: &PgConnection, _form: &StageForm) -> Result<Self, ModelError> {
        let _result = insert_into(job_stages)
            .values(_form)
            .get_result::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }

    fn read(_conn: &PgConnection, _student: &StageId) -> Result<Self, ModelError> {
        let _result = job_stages
            .filter(job_stages::id.eq(_student.clone()))
            .first::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }

    fn update(
        _conn: &PgConnection,
        _event: &StageId,
        _form: &StageForm,
    ) -> Result<Self, ModelError> {
        let _result = diesel::update(job_stages.filter(job_stages::id.eq(_event.clone())))
            .set(_form)
            .get_result::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }

    fn update_only(
        _conn: &PgConnection,
        _event: &StageId,
        _form: &StageForm,
        _columns: Vec<String>,
    ) -> Result<Self, ModelError> {
        let _edited_student = _form.clone();
        Self::update(&_conn, _event, &_edited_student)
    }

    fn delete(_conn: &PgConnection, _event: &StageId) -> Result<usize, ModelError> {
        let _result =
            diesel::delete(job_stages.filter(job_stages::id.eq(_event.clone()))).execute(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl ManagementAsyncTrait<StageForm, StageId> for Stage {
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
                Self::event_by_filter(&s)
            }
            (Some(q), None) => Self::event_by_filter(q),
            (None, Some(sort)) => {
                s.push_str(" 1=1 ORDER BY ");
                s.push_str(&sort.join(", "));
                Self::event_by_filter(&s)
            }
            (None, None) => job_stages.into_boxed(),
        };
        if let Some(total_count) = _total_count {
            if *total_count {
                let _total_sql = match _q {
                    Some(q) => Self::event_by_filter(q),
                    None => job_stages.into_boxed(),
                };
                let _result = _total_sql.select(count_star()).first::<i64>(_conn);
                match _result {
                    Ok(res) => l_return_count = Some(res),
                    Err(_err) => return Err(RecruitError::diesel_error(_err)),
                }
            }
        }
        if let Some(offset) = _offset {
            query = query.offset(*offset);
        }
        if let Some(limit) = _limit {
            query = query.limit(*limit + 1);
        }
        let mut _result = query.load::<Stage>(_conn);
        let mut _data = match _result {
            Ok(res) => res,
            Err(_err) => return Err(RecruitError::diesel_error(_err)),
        };
        let l_has_more = _data.len() > _limit.unwrap() as usize;
        let mut _l_limit: usize;
        if l_has_more {
            _l_limit = _limit.unwrap() as usize;
            _data = (_data[.._l_limit]).to_vec();
        }
        Ok((_data, l_return_count, l_has_more))
    }
    async fn get_item(
        _conn: &PgConnection,
        _event: &StageId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        Stage::read(_conn, _event)
    }
    async fn create_item(
        _conn: &PgConnection,
        _form: &StageForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        Stage::create(_conn, _form)
    }
    async fn update_item(
        _conn: &PgConnection,
        _event: &StageId,
        _form: &StageForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        Stage::update(_conn, _event, _form)
    }
    async fn delete_item(_conn: &PgConnection, _event: &StageId) -> Result<usize, ModelError> {
        Stage::delete(_conn, _event)
    }
}
#[async_trait::async_trait(?Send)]
pub trait Stage_ {
    /// Helper functions
    fn event_by_filter<'a>(_sql: &'a str) -> job_stages::BoxedQuery<'a, diesel::pg::Pg> {
        job_stages::table.filter(sql(_sql)).into_boxed()
    }
    async fn get_items(
        _conn: &PgConnection,
        _stage_id: &Vec<StageId>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _expand: &bool,
    ) -> Result<Vec<StageData>, ModelError>
    where
        Self: Sized;

    async fn get_stage_childs(
        _conn: &PgConnection,
        _stage: &Vec<Stage>,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<
        (
            Option<Vec<Vec<StageAggregation>>>,
            Option<Vec<Vec<Candidate>>>,
            Option<Vec<Vec<Event>>>,
            
        ),
        ModelError,
    >
    where
        Self: Sized;
    async fn get_rubric_aggregations(
        _conn: &PgConnection,
        _stages: &Vec<Stage>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<StageAggregation>>>, ModelError>
    where
        Self: Sized;
    async fn get_stage_candidates(
        _conn: &PgConnection,
        _clients: &Vec<Stage>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
        ) -> Result<Option<Vec<Vec<Candidate>>>, ModelError>
    where
        Self: Sized; 
    async fn get_stage_event(
        _conn: &PgConnection,
        _clients: &Vec<Stage>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
         _expand: &bool,
    ) -> Result<Option<Vec<Vec<Event>>>, ModelError>
    where
        Self: Sized;  
}

#[async_trait::async_trait(?Send)]
impl Stage_ for Stage {
    async fn get_items(
        _conn: &PgConnection,
        _stages_id: &Vec<StageId>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _expand: &bool,
    ) -> Result<Vec<StageData>, ModelError>
    where
        Self: Sized,
    {
        let _result = job_stages::table
            .filter(job_stages::id.eq_any(_stages_id))
            .limit(_limit.unwrap_or(10))
            .offset(_offset.unwrap_or(0))
            .load::<Stage>(_conn);
        match _result {
            Ok(items) => {
                let mut _stage_data = vec![];
                for element in items {
                    let _item = StageData {
                        id: element.id,
                        job_id: element.job_id,
                        type_: element.type_,
                        name: element.name,
                        description: element.description,
                        sequence: element.sequence,
                        metadata: element.metadata,
                        app_metadata: element.app_metadata,
                        created_at: element.created_at,
                        updated_at: element.updated_at,
                        created_by: element.created_by,
                        updated_by: element.updated_by,
                    };
                    _stage_data.push(_item);
                }

                Ok(_stage_data)
            }
            Err(_err) => return Err(RecruitError::diesel_error(_err)),
        }
    }
    async fn get_rubric_aggregations(
        _conn: &PgConnection,
        _stages: &Vec<Stage>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<StageAggregation>>>, ModelError>
    where
        Self: Sized,
    {
        if *_expand {
            let _result = StageAggregation::belonging_to(_stages)
                .limit(_limit.unwrap_or(10))
                .load::<StageAggregation>(_conn);
            match _result {
                Ok(_aggregations) => {
                    let _grouped_aggregations: Vec<Vec<StageAggregation>> =
                        _aggregations.grouped_by(_stages);
                    Ok(Some(_grouped_aggregations))
                }
                Err(_err) => return Err(RecruitError::diesel_error(_err)),
            }
        } else {
            Ok(None)
        }
    }
    async fn get_stage_candidates(
        _conn: &PgConnection,
        _stages: &Vec<Stage>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<Candidate>>>, ModelError>
    where
        Self: Sized,
    {
        if *_expand {
            let _result = Candidate::belonging_to(_stages).load::<Candidate>(_conn);

            match _result {
                Ok(topics) => {
                    let _grouped_topics: Vec<Vec<Candidate>> = topics.grouped_by(_stages);
                    Ok(Some(_grouped_topics))
                }
                Err(_err) => return Err(RecruitError::diesel_error(_err)),
            }
        } else {
            Ok(None)
        }
    }
    async fn get_stage_event(
        _conn: &PgConnection,
        _stages: &Vec<Stage>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<Event>>>, ModelError>
    where
        Self: Sized,
    {
        if *_expand {
            let _result = Event::belonging_to(_stages).load::<Event>(_conn);

            match _result {
                Ok(topics) => {
                    let _grouped_topics: Vec<Vec<Event>> = topics.grouped_by(_stages);
                    Ok(Some(_grouped_topics))
                }
                Err(_err) => return Err(RecruitError::diesel_error(_err)),
            }
        } else {
            Ok(None)
        }
    }

    async fn get_stage_childs(
        _conn: &PgConnection,
        _stages: &Vec<Stage>,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<
        (
            Option<Vec<Vec<StageAggregation>>>,
            Option<Vec<Vec<Candidate>>>,
            Option<Vec<Vec<Event>>>,
        ),
        ModelError,
    > {
        let _return = match _expand {
            Some(val) => {
                let exec_plan: (bool, bool, bool) =
                    if val.iter().any(|a| a == "all") {
                        (true, true, true)
                    } else {
                        (
                            val.iter().any(|a| a == "aggregations"),
                            val.iter().any(|a| a == "candidates"),
                            val.iter().any(|a| a == "events"),
                        )
                    };
                let aggregations_fut = Stage::get_rubric_aggregations(
                    _conn,
                    _stages,
                    &None,
                    &None,
                    &None,
                    &exec_plan.0,
                );
                let candidate_fut = Stage::get_stage_candidates(
                    _conn,
                    _stages,
                    &None,
                    &None,
                    &None,
                    &exec_plan.1,
                );
                let event_fut = Stage::get_stage_event(
                    _conn,
                    _stages,
                    &None,
                    &None,
                    &None,
                    &exec_plan.2,
                );

                try_join!(
                    aggregations_fut,
                    candidate_fut,
                    event_fut,
                )?
            }
            None => (None, None, None),
        };
        Ok(_return)
    }
}

#[async_trait::async_trait(?Send)]
impl ManagementAsyncTrait<StageForm, StageId> for StageData {}