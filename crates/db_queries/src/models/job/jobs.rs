use db_schema::{models::job::{jobs::{Job, JobForm, JobAggregation}, job_candidates::Candidate, members::Member, sources::Sources, stages::Stage, events::Event}, JobId, schema::jobs::{self, dsl::*}, RecruitError};
use diesel::{
    dsl::{count_star, sql},
    insert_into,
    sql_types::Bool,
    PgConnection, QueryDsl, RunQueryDsl,
};
use crate::{
    diesel::{BelongingToDsl, ExpressionMethods, GroupedBy},
    Crud, ManagementAsyncTrait,
};
use errors_lib_rs::{db::ModelErrorMessage, model::ModelError};

use futures::try_join;

impl Crud<JobForm, JobId> for Job {
    fn create(_conn: &PgConnection, _form: &JobForm) -> Result<Self, ModelError> {

        let _result = insert_into(jobs)
            .values(_form)
            .get_result::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }
    fn read(_conn: &PgConnection, _code: &JobId) -> Result<Job, ModelError> {
        let _result = jobs.filter(id.eq(_code)).first::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }
    fn update(
        _conn: &PgConnection,
        _code: &JobId,
        _form: &JobForm,
    ) -> Result<Self, ModelError> {
        let _result = diesel::update(jobs.filter(id.eq(_code)))
            .set(_form)
            .get_result::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }
    fn update_only(
        _conn: &PgConnection,
        _code: &JobId,
        _form: &JobForm,
        _columns: Vec<String>,
    ) -> Result<Self, ModelError> {
        let edited_tree = _form.clone();
        Self::update(&_conn, _code, &edited_tree)
    }
    fn delete(_conn: &PgConnection, _code: &JobId) -> Result<usize, ModelError> {
        let _result = diesel::delete(jobs.filter(id.eq(_code))).execute(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(RecruitError::diesel_error(_err)),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl ManagementAsyncTrait<JobForm, JobId> for Job {
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
                Self::job_by_filter(&s)
            }
            (Some(q), None) => Self::job_by_filter(q),
            (None, Some(sort)) => {
                s.push_str(" 1=1 ORDER BY ");
                s.push_str(&sort.join(", "));
                Self::job_by_filter(&s)
            }
            (None, None) => jobs.into_boxed(),
        };
        if let Some(total_count) = _total_count {
            if *total_count {
                let _total_sql = match _q {
                    Some(q) => Self::job_by_filter(q),
                    None => jobs.into_boxed(),
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
        let mut _result = query.load::<Job>(_conn);

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
        _code: &JobId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        Job::read(_conn, &_code)
    }

    async fn create_item(
        _conn: &PgConnection,
        _form: &JobForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        Job::create(_conn, _form)
    }
    async fn update_item(
        _conn: &PgConnection,
        _code: &JobId,
        _form: &JobForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        Job::update(_conn, _code, _form)
    }
    async fn delete_item(_conn: &PgConnection, _code: &JobId) -> Result<usize, ModelError> {
        Job::delete(_conn, _code)
    }
}
#[async_trait::async_trait(?Send)]
pub trait Job_ {
    fn job_by_filter<'a>(_sql: &'a str) -> jobs::BoxedQuery<'a, diesel::pg::Pg> {
        jobs::table.filter(sql::<Bool>(_sql)).into_boxed()
    }

    fn get_job_id(
        _conn: &PgConnection,
        _code: JobId,
    ) -> Result<JobId, ModelError> {
        let _result = jobs
            .filter(id.eq(_code))
            .select(id)
            .first::<JobId>(_conn);
        match _result {
            Ok(_res) => Ok(_res),
            Err(_err) => return Err(RecruitError::diesel_error(_err)),
        }
    }
    async fn get_items(
        _conn: &PgConnection,
        _gradebooks_id: &Vec<JobId>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
    ) -> Result<Vec<Job>, ModelError>
    where
        Self: Sized;

    async fn get_job_aggregation(
        _conn: &PgConnection,
        _jobs: &Vec<Job>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<JobAggregation>>>, ModelError>
    where
        Self: Sized;

    async fn get_job_candidate(
        _conn: &PgConnection,
        _jobs: &Vec<Job>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<Candidate>>>, ModelError>
    where
        Self: Sized;

    async fn get_member(
        _conn: &PgConnection,
        _jobs: &Vec<Job>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<Member>>>, ModelError>
    where
        Self: Sized;

    async fn get_event(
        _conn: &PgConnection,
        _jobs: &Vec<Job>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<Event>>>, ModelError>
    where
        Self: Sized;

    async fn get_source(
        _conn: &PgConnection,
        _jobs: &Vec<Job>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<Sources>>>, ModelError>
    where
        Self: Sized;

    async fn get_stage(
        _conn: &PgConnection,
        _jobs: &Vec<Job>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<Stage>>>, ModelError>
    where
        Self: Sized;

    async fn get_job_childs(
        _conn: &PgConnection,
        _jobs: &Vec<Job>,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<
        (
            Option<Vec<Vec<JobAggregation>>>,
            Option<Vec<Vec<Stage>>>,
            Option<Vec<Vec<Sources>>>,
            Option<Vec<Vec<Candidate>>>,
            Option<Vec<Vec<Event>>>,
            Option<Vec<Vec<Member>>>,
        ),
        ModelError,
    >
    where
        Self: Sized;
}

#[async_trait::async_trait(?Send)]
impl Job_ for Job {

    async fn get_items(
        _conn: &PgConnection,
        _job_id: &Vec<JobId>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
    ) -> Result<Vec<Job>, ModelError>
    where
        Self: Sized,
    {
        let _result = jobs::table
            .filter(jobs::id.eq_any(_job_id))
            .limit(_limit.unwrap_or(10))
            .offset(_offset.unwrap_or(0))
            .load::<Job>(_conn);
        match _result {
            Ok(items) => {
                let mut _job_data = vec![];
                for element in items {
                    let _item = Job {
                        id: element.id,
                        name: element.name,
                        description: element.description,
                        requirements: element.requirements,
                        workload: element.workload,
                        temporary: element.temporary,
                        function: element.function,
                        department: element.department,
                        location: element.location,
                        industry: element.industry,
                        benefits: element.benefits,
                        salary: element.salary,
                        metadata: element.metadata,
                        status: element.status,
                        app_metadata: element.app_metadata,
                        created_by: element.created_by,
                        created_at: element.created_at,
                        updated_by: element.updated_by,
                        updated_at: element.updated_at,
                    };
                    _job_data.push(_item);
                }

                Ok(_job_data)
            }
            Err(_err) => return Err(RecruitError::diesel_error(_err)),
        }
    }
    async fn get_job_aggregation(
        _conn: &PgConnection,
        _jobs: &Vec<Job>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<JobAggregation>>>, ModelError>
    where
        Self: Sized,
    {
        if *_expand {
            let _result = JobAggregation::belonging_to(_jobs)
                .limit(_limit.unwrap_or(10))
                .load::<JobAggregation>(_conn);
            match _result {
                Ok(res) => {
                    let _grouped_aggrigations: Vec<Vec<JobAggregation>> =
                        res.grouped_by(_jobs);
                    Ok(Some(_grouped_aggrigations))
                }
                Err(_err) => return Err(RecruitError::diesel_error(_err)),
            }
        } else {
            Ok(None)
        }
    }
    async fn get_job_candidate(
        _conn: &PgConnection,
        _jobs: &Vec<Job>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<Candidate>>>, ModelError>
    where
        Self: Sized,
    {
        if *_expand {
            let _result = Candidate::belonging_to(_jobs)
                .limit(_limit.unwrap_or(10))
                .load::<Candidate>(_conn);
            match _result {
                Ok(res) => {
                    let _grouped_job: Vec<Vec<Candidate>> = res.grouped_by(_jobs);
                    Ok(Some(_grouped_job))
                }
                Err(_err) => return Err(RecruitError::diesel_error(_err)),
            }
        } else {
            Ok(None)
        }
    }
    async fn get_member(
        _conn: &PgConnection,
        _jobs: &Vec<Job>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<Member>>>, ModelError>
    where
        Self: Sized,
    {
        if *_expand {
            let _result = Member::belonging_to(_jobs)
                .limit(_limit.unwrap_or(10))
                .load::<Member>(_conn);
            match _result {
                Ok(res) => {
                    let _grouped_version: Vec<Vec<Member>> = res.grouped_by(_jobs);
                    Ok(Some(_grouped_version))
                }
                Err(_err) => return Err(RecruitError::diesel_error(_err)),
            }
        } else {
            Ok(None)
        }
    }
    async fn get_event(
        _conn: &PgConnection,
        _jobs: &Vec<Job>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<Event>>>, ModelError>
    where
        Self: Sized,
    {
        if *_expand {
            let _result = Event::belonging_to(_jobs)
                .limit(_limit.unwrap_or(10))
                .load::<Event>(_conn);
            match _result {
                Ok(res) => {
                    let _grouped_version: Vec<Vec<Event>> = res.grouped_by(_jobs);
                    Ok(Some(_grouped_version))
                }
                Err(_err) => return Err(RecruitError::diesel_error(_err)),
            }
        } else {
            Ok(None)
        }
    }
    async fn get_source(
        _conn: &PgConnection,
        _jobs: &Vec<Job>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<Sources>>>, ModelError>
    where
        Self: Sized,
    {
        if *_expand {
            let _result = Sources::belonging_to(_jobs)
                .limit(_limit.unwrap_or(10))
                .load::<Sources>(_conn);
            match _result {
                Ok(res) => {
                    let _grouped_version: Vec<Vec<Sources>> = res.grouped_by(_jobs);
                    Ok(Some(_grouped_version))
                }
                Err(_err) => return Err(RecruitError::diesel_error(_err)),
            }
        } else {
            Ok(None)
        }
    }
    async fn get_stage(
        _conn: &PgConnection,
        _jobs: &Vec<Job>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _fields: &Option<Vec<String>>,
        _expand: &bool,
    ) -> Result<Option<Vec<Vec<Stage>>>, ModelError>
    where
        Self: Sized,
    {
        if *_expand {
            let _result = Stage::belonging_to(_jobs)
                .limit(_limit.unwrap_or(10))
                .load::<Stage>(_conn);
            match _result {
                Ok(res) => {
                    let _grouped_version: Vec<Vec<Stage>> = res.grouped_by(_jobs);
                    Ok(Some(_grouped_version))
                }
                Err(_err) => return Err(RecruitError::diesel_error(_err)),
            }
        } else {
            Ok(None)
        }
    }
    
    async fn get_job_childs(
        _conn: &PgConnection,
        _jobs: &Vec<Job>,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<
        (
            Option<Vec<Vec<JobAggregation>>>,
            Option<Vec<Vec<Stage>>>,
            Option<Vec<Vec<Sources>>>,
            Option<Vec<Vec<Candidate>>>,
            Option<Vec<Vec<Event>>>,
            Option<Vec<Vec<Member>>>,
        ),
        ModelError,
    > {
        let _return = match _expand {
            Some(val) => {
                let exec_plan: (bool, bool, bool, bool, bool, bool) = if val.iter().any(|a| a == "all") {
                    (true, true, true, true, true, true)
                } else {
                    (
                        val.iter().any(|a| a == "aggregations"),
                        val.iter().any(|a| a == "stages"),
                        val.iter().any(|a| a == "sources"),
                        val.iter().any(|a| a == "candidates"),
                        val.iter().any(|a| a == "events"),
                        val.iter().any(|a| a == "members"),
                    )
                };
                let aggregations_fut = Job::get_job_aggregation(
                    _conn,
                    _jobs,
                    &None,
                    &None,
                    &None,
                    &exec_plan.0,
                );
                let stage_fut =
                    Job::get_stage(_conn, _jobs, &None, &None, &None, &exec_plan.1);
                let source_fut =
                    Job::get_source(_conn, _jobs, &None, &None, &None, &exec_plan.2);
                let candidate_fut =
                    Job::get_job_candidate(_conn, _jobs, &None, &None, &None, &exec_plan.3);
                let event_fut =
                    Job::get_event(_conn, _jobs, &None, &None, &None, &exec_plan.4);
                let member_fut =
                    Job::get_member(_conn, _jobs, &None, &None, &None, &exec_plan.5);

                try_join!(aggregations_fut, stage_fut, source_fut, candidate_fut, event_fut, member_fut)?
            }
            None => (None, None, None, None, None, None),
        };
        Ok(_return)
    }
}
