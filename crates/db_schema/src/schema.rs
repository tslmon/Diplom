// @generated automatically by Diesel CLI.

diesel::table! {
    job_aggregations (id) {
        id -> Varchar,
        job_id -> Varchar,
        stages -> Int8,
        sources -> Int8,
        candidates -> Int8,
        events -> Int8,
        members -> Int8,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    job_candidate_aggregations (id) {
        id -> Varchar,
        candidate_id -> Varchar,
        events -> Int8,
        offers -> Int8,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    job_candidates (id) {
        id -> Varchar,
        job_id -> Varchar,
        stage_id -> Varchar,
        source_id -> Varchar,
        user_id -> Varchar,
        profile_id -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        referred_user_id -> Varchar,
        originating_candidate_id -> Varchar,
        disqualified -> Nullable<Bool>,
        disqualified_by -> Nullable<Varchar>,
        disqualified_at -> Nullable<Date>,
        disqualified_reason -> Nullable<Varchar>,
        status -> Varchar,
        metadata -> Jsonb,
        app_metadata -> Jsonb,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    job_event_aggregations (id) {
        id -> Varchar,
        event_id -> Varchar,
        organizers -> Int8,
        candidates -> Int8,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    job_event_candidates (id) {
        id -> Varchar,
        event_id -> Varchar,
        candidate_id -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    job_event_organizers (id) {
        id -> Varchar,
        event_id -> Varchar,
        member_id -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    job_events (id) {
        id -> Varchar,
        job_id -> Varchar,
        stage_id -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        sequence -> Int8,
        metadata -> Jsonb,
        app_metadata -> Jsonb,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {    
    job_members (id) {
        id -> Varchar,
        job_id -> Varchar,
        user_role -> Varchar,
        user_id -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    job_offer_aggregations (id) {
        id -> Varchar,
        offer_id -> Varchar,
        variables -> Int8,
        documents -> Int8,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    job_offer_documents (id) {
        id -> Varchar,
        offer_id -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        file_id -> Varchar,
        file_name -> Varchar,
        file_type -> Varchar,
        file_size -> Nullable<Int8>,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    job_offer_variables (variable) {
        offer_id -> Varchar,
        variable -> Varchar,
        value -> Nullable<Date>,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    job_offers (id) {
        id -> Varchar,
        job_id -> Varchar,
        candidate_id -> Varchar,
        title -> Varchar,
        body -> Nullable<Text>,
        status -> Varchar,
        metadata -> Jsonb,
        app_metadata -> Jsonb,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    job_source_aggregations (id) {
        id -> Varchar,
        source_id -> Varchar,
        candidates -> Int8,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    job_sources (id) {
        id -> Varchar,
        job_id -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        metadata -> Jsonb,
        app_metadata -> Jsonb,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    job_stage_aggregations (id) {
        id -> Varchar,
        stage_id -> Varchar,
        candidates -> Int8,
        events -> Int8,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    job_stages (id) {
        id -> Varchar,
        job_id -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        sequence -> Int8,
        locked -> Nullable<Bool>,
        metadata -> Jsonb,
        app_metadata -> Jsonb,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    jobs (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        requirements -> Varchar,
        workload -> Varchar,
        temporary -> Nullable<Bool>,
        function -> Varchar,
        department -> Varchar,
        location -> Varchar,
        industry -> Varchar,
        benefits -> Varchar,
        salary -> Jsonb,
        metadata -> Jsonb,
        status -> Varchar,
        app_metadata -> Jsonb,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    pool_aggregations (id) {
        id -> Varchar,
        pool_id -> Varchar,
        members -> Int8,
        candidates -> Int8,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    pool_candidates (id) {
        id -> Varchar,
        pool_id -> Varchar,
        user_id -> Varchar,
        profile_id -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        originating_candidate_id -> Varchar,
        metadata -> Jsonb,
        app_metadata -> Jsonb,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    pool_members (id) {
        id -> Varchar,
        pool_id -> Varchar,
        user_role -> Varchar,
        user_id -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    pools (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        metadata -> Jsonb,
        app_metadata -> Jsonb,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(job_aggregations -> jobs (job_id));
diesel::joinable!(job_candidate_aggregations -> job_candidates (candidate_id));
diesel::joinable!(job_candidates -> job_sources (source_id));
diesel::joinable!(job_candidates -> job_stages (stage_id));
diesel::joinable!(job_candidates -> jobs (job_id));
diesel::joinable!(job_event_aggregations -> job_events (event_id));
diesel::joinable!(job_event_candidates -> job_candidates (candidate_id));
diesel::joinable!(job_event_candidates -> job_events (event_id));
diesel::joinable!(job_event_organizers -> job_events (event_id));
diesel::joinable!(job_event_organizers -> job_members (member_id));
diesel::joinable!(job_events -> job_stages (stage_id));
diesel::joinable!(job_events -> jobs (job_id));
diesel::joinable!(job_members -> jobs (job_id));
diesel::joinable!(job_offer_aggregations -> job_offers (offer_id));
diesel::joinable!(job_offer_documents -> job_offers (offer_id));
diesel::joinable!(job_offer_variables -> job_offers (offer_id));
diesel::joinable!(job_offers -> job_candidates (candidate_id));
diesel::joinable!(job_offers -> jobs (job_id));
diesel::joinable!(job_source_aggregations -> job_sources (source_id));
diesel::joinable!(job_sources -> jobs (job_id));
diesel::joinable!(job_stage_aggregations -> job_stages (stage_id));
diesel::joinable!(job_stages -> jobs (job_id));
diesel::joinable!(pool_aggregations -> pools (pool_id));
diesel::joinable!(pool_candidates -> pools (pool_id));
diesel::joinable!(pool_members -> pools (pool_id));

diesel::allow_tables_to_appear_in_same_query!(
    job_aggregations,
    job_candidate_aggregations,
    job_candidates,
    job_event_aggregations,
    job_event_candidates,
    job_event_organizers,
    job_events,
    job_members,
    job_offer_aggregations,
    job_offer_documents,
    job_offer_variables,
    job_offers,
    job_source_aggregations,
    job_sources,
    job_stage_aggregations,
    job_stages,
    jobs,
    pool_aggregations,
    pool_candidates,
    pool_members,
    pools,
);
