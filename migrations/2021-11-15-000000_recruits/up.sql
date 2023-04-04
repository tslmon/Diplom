

--
--  jobs
--

CREATE TABLE IF NOT EXISTS jobs (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    name character varying(255) NOT NULL,
    description text,
    requirements character varying(255) NOT NULL,    
    workload character varying(255) NOT NULL,  
    temporary BOOLEAN DEFAULT false,  
    function character varying(255) NOT NULL,  
    department character varying(255) NOT NULL,  
    location character varying(255) NOT NULL,  
    industry character varying(255) NOT NULL,  
    benefits character varying(255) NOT NULL,  
    salary jsonb NOT NULL DEFAULT '{}'::jsonb,    
    metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
    status character varying(255) NOT NULL,
    app_metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_jobs PRIMARY KEY (id),
    CONSTRAINT ukey_job_name UNIQUE (name)
);

SELECT diesel_manage_updated_at('jobs');

--
-- job_sources
--

CREATE TABLE IF NOT EXISTS job_sources (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    job_id character varying(255) NOT NULL,
    type character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    description text,
    metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
    app_metadata jsonb NOT NULL DEFAULT '{}'::jsonb,    
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_job_sources PRIMARY KEY (id),
    CONSTRAINT fkey_job_sources_jobs FOREIGN KEY (job_id)
        REFERENCES jobs (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('job_sources');

--
-- job_stages
--

CREATE TABLE IF NOT EXISTS job_stages (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    job_id character varying(255) NOT NULL,
    type character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    description text,
    sequence bigint NOT NULL DEFAULT 1,
    locked BOOLEAN DEFAULT false,
    metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
    app_metadata jsonb NOT NULL DEFAULT '{}'::jsonb,      
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_job_stages PRIMARY KEY (id),
    CONSTRAINT fkey_job_stages_jobs FOREIGN KEY (job_id)
        REFERENCES jobs (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('job_stages');

--
-- job_candidates
--

CREATE TABLE IF NOT EXISTS job_candidates (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    job_id character varying(255) NOT NULL,
    stage_id character varying(255) NOT NULL,
    source_id character varying(255) NOT NULL,
    user_id character varying(255) NOT NULL,
    profile_id character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    description text,
    referred_user_id character varying(255) NOT NULL,
    originating_candidate_id character varying(255) NOT NULL,
    disqualified BOOLEAN DEFAULT false,
    disqualified_by  character varying(255) DEFAULT NULL,
    disqualified_at  date DEFAULT NULL,
    disqualified_reason  character varying(255) DEFAULT NULL,
    status character varying(255) NOT NULL,
    metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
    app_metadata jsonb NOT NULL DEFAULT '{}'::jsonb,      
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_job_candidates PRIMARY KEY (id),
    CONSTRAINT fkey_job_candidates_jobs FOREIGN KEY (job_id)
        REFERENCES jobs (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,  
    CONSTRAINT fkey_job_candidates_job_stages FOREIGN KEY (stage_id)
        REFERENCES job_stages (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE, 
    CONSTRAINT fkey_job_candidates_job_sources FOREIGN KEY (source_id)
        REFERENCES job_sources (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE                
);

SELECT diesel_manage_updated_at('job_candidates');

--
-- job_offers
--

CREATE TABLE IF NOT EXISTS job_offers (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    job_id character varying(255) NOT NULL,
    candidate_id character varying(255) NOT NULL,
    title character varying(255) NOT NULL,
    body text,
    status character varying(255) NOT NULL,
    metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
    app_metadata jsonb NOT NULL DEFAULT '{}'::jsonb,      
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_job_offers PRIMARY KEY (id),
    CONSTRAINT fkey_job_offers_jobs FOREIGN KEY (job_id)
        REFERENCES jobs (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT fkey_job_offers_job_candidates FOREIGN KEY (candidate_id)
        REFERENCES job_candidates (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE        
);

SELECT diesel_manage_updated_at('job_offers');

--
-- job_members
--

CREATE TABLE IF NOT EXISTS job_members (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    job_id character varying(255) NOT NULL,
    user_role character varying(255) NOT NULL,
    user_id character varying(255) NOT NULL,  
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_job_members PRIMARY KEY (id),
    CONSTRAINT fkey_job_members_jobs FOREIGN KEY (job_id)
        REFERENCES jobs (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('job_members');

--
-- job_events
--

CREATE TABLE IF NOT EXISTS job_events (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    job_id character varying(255) NOT NULL,
    stage_id character varying(255) NOT NULL,    
    type character varying(255) NOT NULL,
    name character varying(255) NOT NULL,
    description text,
    sequence bigint NOT NULL DEFAULT 1,
    metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
    app_metadata jsonb NOT NULL DEFAULT '{}'::jsonb,      
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_job_events PRIMARY KEY (id),
    CONSTRAINT fkey_job_events_jobs FOREIGN KEY (job_id)
        REFERENCES jobs (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT fkey_job_events_job_stages FOREIGN KEY (stage_id)
        REFERENCES job_stages (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE        
);

SELECT diesel_manage_updated_at('job_events');

--
-- job_offer_variables
--

CREATE TABLE IF NOT EXISTS job_offer_variables (
    offer_id character varying(255) NOT NULL,
    variable character varying(255) NOT NULL,
    value date,  
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_job_offer_variables PRIMARY KEY (variable),
    CONSTRAINT fkey_job_offer_variables_job_offers FOREIGN KEY (offer_id)
        REFERENCES job_offers (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('job_offer_variables');

--
-- job_offer_documents
--

CREATE TABLE IF NOT EXISTS job_offer_documents (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),  
    offer_id character varying(255) NOT NULL,
    type character varying(255) NOT NULL,
    file_id character varying(255) NOT NULL,
    file_name character varying(255) NOT NULL,
    file_type character varying(255) NOT NULL,              
    file_size bigint,  
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_job_offer_documents PRIMARY KEY (id),
    CONSTRAINT fkey_job_offer_documents_job_offers FOREIGN KEY (offer_id)
        REFERENCES job_offers (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('job_offer_documents');

--
-- job_event_candidates
--

CREATE TABLE IF NOT EXISTS job_event_candidates (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    event_id character varying(255) NOT NULL,
    candidate_id character varying(255) NOT NULL,  
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_job_event_candidates PRIMARY KEY (id),
    CONSTRAINT fkey_job_event_candidates_job_events FOREIGN KEY (event_id)
        REFERENCES job_events (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT fkey_job_event_candidates_job_candidates FOREIGN KEY (candidate_id)
        REFERENCES job_candidates (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('job_event_candidates');

--
-- job_event_organizers
--

CREATE TABLE IF NOT EXISTS job_event_organizers (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    event_id character varying(255) NOT NULL,
    member_id character varying(255) NOT NULL,  
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_job_event_organizers PRIMARY KEY (id),
    CONSTRAINT fkey_job_event_organizers_job_events FOREIGN KEY (event_id)
        REFERENCES job_events (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT fkey_job_event_organizers_job_members FOREIGN KEY (member_id)
        REFERENCES job_members(id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('job_event_organizers');

--
--  pools
--

CREATE TABLE IF NOT EXISTS pools (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    name character varying(255) NOT NULL,
    description text,
    metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
    app_metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_pools PRIMARY KEY (id),
    CONSTRAINT ukey_pool_name UNIQUE (name)
);

SELECT diesel_manage_updated_at('pools');

--
-- pool_members
--

CREATE TABLE IF NOT EXISTS pool_members (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    pool_id character varying(255) NOT NULL,
    user_role character varying(255) NOT NULL,  
    user_id character varying(255) NOT NULL,      
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_pool_members PRIMARY KEY (id),
    CONSTRAINT fkey_pool_members_pools FOREIGN KEY (pool_id)
        REFERENCES pools (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('pool_members');

--
-- pool_candidates
--

CREATE TABLE IF NOT EXISTS pool_candidates (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    pool_id character varying(255) NOT NULL, 
    user_id character varying(255) NOT NULL,   
    profile_id character varying(255) NOT NULL,  
    name character varying(255) NOT NULL, 
    description text, 
    originating_candidate_id character varying(255) NOT NULL,      
    metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
    app_metadata jsonb NOT NULL DEFAULT '{}'::jsonb,     
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_pool_candidates PRIMARY KEY (id),
    CONSTRAINT fkey_pool_candidates_pools FOREIGN KEY (pool_id)
        REFERENCES pools (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('pool_candidates');

--
-- ////// Aggregations. //////
--

--
-- jobs Aggregations
--

CREATE TABLE IF NOT EXISTS job_aggregations (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    job_id character varying(255) NOT NULL,
    stages bigint NOT NULL DEFAULT 0,
    sources bigint NOT NULL DEFAULT 0,
    candidates bigint NOT NULL DEFAULT 0,
    events bigint NOT NULL DEFAULT 0,
    members bigint NOT NULL DEFAULT 0,    
    created_by character varying(255) NOT NULL,    
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,        
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_job_aggregations PRIMARY KEY (id),
    CONSTRAINT fkey_job_aggregations_jobs FOREIGN KEY (job_id)
        REFERENCES jobs (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);


CREATE INDEX idx_job_aggregations_stages
    ON job_aggregations USING btree
    (stages DESC NULLS FIRST);

CREATE INDEX idx_job_aggregations_sources
    ON job_aggregations USING btree
    (sources DESC NULLS FIRST);

CREATE INDEX idx_job_aggregations_candidates
    ON job_aggregations USING btree
    (candidates DESC NULLS FIRST);

CREATE INDEX idx_job_aggregations_events
    ON job_aggregations USING btree
    (events DESC NULLS FIRST);

CREATE INDEX idx_job_aggregations_members
    ON job_aggregations USING btree
    (members DESC NULLS FIRST);

SELECT diesel_manage_updated_at('job_aggregations');

--
-- job_source_aggregations
--

CREATE TABLE IF NOT EXISTS job_source_aggregations (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    source_id character varying(255) NOT NULL,
    candidates bigint NOT NULL DEFAULT 0,
    created_by character varying(255) NOT NULL,  
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,     
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_job_source_aggregations PRIMARY KEY (id),
    CONSTRAINT fkey_job_source_aggregations_job_sources FOREIGN KEY (source_id)
        REFERENCES job_sources (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE INDEX idx_job_source_aggregations_candidates
    ON job_source_aggregations USING btree
    (candidates DESC NULLS FIRST);

--
-- job_stage_aggregations
--

CREATE TABLE IF NOT EXISTS job_stage_aggregations (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    stage_id character varying(255) NOT NULL,
    candidates bigint NOT NULL DEFAULT 0,
    events bigint NOT NULL DEFAULT 0,    
    created_by character varying(255) NOT NULL,  
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_job_stage_aggregations PRIMARY KEY (id),
    CONSTRAINT fkey_job_stage_aggregations_job_stages FOREIGN KEY (stage_id)
        REFERENCES job_stages (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE INDEX idx_job_stage_aggregations_candidates
    ON job_stage_aggregations USING btree
    (candidates DESC NULLS FIRST);

CREATE INDEX idx_job_stage_aggregations_events
    ON job_stage_aggregations USING btree
    (events DESC NULLS FIRST);

--
-- job_offer_aggregations
--

CREATE TABLE IF NOT EXISTS job_offer_aggregations (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    offer_id character varying(255) NOT NULL,
    variables bigint NOT NULL DEFAULT 0,
    documents bigint NOT NULL DEFAULT 0,    
    created_by character varying(255) NOT NULL,  
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_job_offer_aggregations PRIMARY KEY (id),
    CONSTRAINT fkey_job_offer_aggregations_job_offers FOREIGN KEY (offer_id)
        REFERENCES job_offers (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE INDEX idx_job_offer_aggregations_variables
    ON job_offer_aggregations USING btree
    (variables DESC NULLS FIRST);

CREATE INDEX idx_job_offer_aggregations_documents
    ON job_offer_aggregations USING btree
    (documents DESC NULLS FIRST);

--
-- job_candidate_aggregations
--

CREATE TABLE IF NOT EXISTS job_candidate_aggregations (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    candidate_id character varying(255) NOT NULL,
    events bigint NOT NULL DEFAULT 0,
    offers bigint NOT NULL DEFAULT 0,    
    created_by character varying(255) NOT NULL,  
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_job_candidate_aggregations PRIMARY KEY (id),
    CONSTRAINT fkey_job_candidate_aggregations_job_candidates FOREIGN KEY (candidate_id)
        REFERENCES job_candidates (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE INDEX idx_job_candidate_aggregations_events
    ON job_candidate_aggregations USING btree
    (events DESC NULLS FIRST);

CREATE INDEX idx_job_candidate_aggregations_offers
    ON job_candidate_aggregations USING btree
    (offers DESC NULLS FIRST);

--
-- job_event_aggregations
--

CREATE TABLE IF NOT EXISTS job_event_aggregations (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    event_id character varying(255) NOT NULL,
    organizers bigint NOT NULL DEFAULT 0,
    candidates bigint NOT NULL DEFAULT 0, 
    created_by character varying(255) NOT NULL,     
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_job_event_aggregations PRIMARY KEY (id),
    CONSTRAINT fkey_job_event_aggregations_job_events FOREIGN KEY (event_id)
        REFERENCES job_events (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE INDEX idx_job_event_aggregations_organizers
    ON job_event_aggregations USING btree
    (organizers DESC NULLS FIRST);

CREATE INDEX idx_job_event_aggregations_candidates
    ON job_event_aggregations USING btree
    (candidates DESC NULLS FIRST);

--
-- pool_aggregations
--

CREATE TABLE IF NOT EXISTS pool_aggregations (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    pool_id character varying(255) NOT NULL,
    members bigint NOT NULL DEFAULT 0,
    candidates bigint NOT NULL DEFAULT 0,  
    created_by character varying(255) NOT NULL,    
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_pool_aggregations PRIMARY KEY (id),
    CONSTRAINT fkey_pool_aggregations_pools FOREIGN KEY (pool_id)
        REFERENCES pools (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE INDEX idx_pool_aggregations_members
    ON pool_aggregations USING btree
    (members DESC NULLS FIRST);

CREATE INDEX idx_pool_aggregations_candidates
    ON pool_aggregations USING btree
    (candidates DESC NULLS FIRST);

--
--  ////////////////// aggragation main
--

--
--  job Aggregations main function.
--

CREATE OR REPLACE FUNCTION job_aggregations_main()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    insert into job_aggregations (job_id, created_by, updated_by) values (NEW.id, NEW.created_by, NEW.updated_by);
  ELSIF (TG_OP = 'DELETE') THEN
    delete from job_aggregations where job_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

--
--  job trigger for job_aggregations.
--

CREATE OR REPLACE TRIGGER job_aggregations_main 
    AFTER INSERT OR DELETE
    ON jobs
    FOR EACH ROW
    EXECUTE PROCEDURE job_aggregations_main();


--
--  job_source Aggregations main function.
--

CREATE OR REPLACE FUNCTION job_source_aggregations_main()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    insert into job_source_aggregations (source_id, created_by, updated_by) values (NEW.id, NEW.created_by, NEW.updated_by);
  ELSIF (TG_OP = 'DELETE') THEN
    delete from job_source_aggregations where source_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

--
--  job_source trigger for source_aggregations.
--

CREATE OR REPLACE TRIGGER job_source_aggregations_main 
    AFTER INSERT OR DELETE
    ON job_sources
    FOR EACH ROW
    EXECUTE PROCEDURE job_source_aggregations_main();    


--
--  job_stage Aggregations main function.
--

CREATE OR REPLACE FUNCTION job_stage_aggregations_main()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    insert into job_stage_aggregations (stage_id, created_by, updated_by) values (NEW.id, NEW.created_by, NEW.updated_by);
  ELSIF (TG_OP = 'DELETE') THEN
    delete from job_stage_aggregations where stage_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

--
--  job_stage trigger for stage_aggregations.
--

CREATE OR REPLACE TRIGGER job_stage_aggregations_main 
    AFTER INSERT OR DELETE
    ON job_stages
    FOR EACH ROW
    EXECUTE PROCEDURE job_stage_aggregations_main();   

--
--  job_offer Aggregations main function.
--

CREATE OR REPLACE FUNCTION job_offer_aggregations_main()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    insert into job_offer_aggregations (offer_id ,created_by, updated_by) values (NEW.id, NEW.created_by, NEW.updated_by);
  ELSIF (TG_OP = 'DELETE') THEN
    delete from job_offer_aggregations where offer_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

--
--  job_offer trigger for stage_aggregations.
--

CREATE OR REPLACE TRIGGER job_offer_aggregations_main 
    AFTER INSERT OR DELETE
    ON job_offers
    FOR EACH ROW
    EXECUTE PROCEDURE job_offer_aggregations_main(); 


--
--  job_candidate Aggregations main function.
--

CREATE OR REPLACE FUNCTION job_candidate_aggregations_main()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    insert into job_candidate_aggregations (candidate_id ,created_by, updated_by) values (NEW.id, NEW.created_by, NEW.updated_by);
  ELSIF (TG_OP = 'DELETE') THEN
    delete from job_candidate_aggregations where candidate_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

--
--  job_candidate trigger for candidate_aggregations.
--

CREATE OR REPLACE TRIGGER job_candidate_aggregations_main 
    AFTER INSERT OR DELETE
    ON job_candidates
    FOR EACH ROW
    EXECUTE PROCEDURE job_candidate_aggregations_main();      

--
--  job_event Aggregations main function.
--

CREATE OR REPLACE FUNCTION job_event_aggregations_main()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    insert into job_event_aggregations (event_id ,created_by, updated_by) values (NEW.id, NEW.created_by, NEW.updated_by);
  ELSIF (TG_OP = 'DELETE') THEN
    delete from job_event_aggregations where event_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

--
--  job_event trigger for event_aggregations.
--

CREATE OR REPLACE TRIGGER job_event_aggregations_main 
    AFTER INSERT OR DELETE
    ON job_events
    FOR EACH ROW
    EXECUTE PROCEDURE job_event_aggregations_main();  

--
--  pool Aggregations main function.
--

CREATE OR REPLACE FUNCTION pool_aggregations_main()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    insert into pool_aggregations (pool_id, created_by, updated_by) values (NEW.id ,NEW.created_by, NEW.updated_by);
  ELSIF (TG_OP = 'DELETE') THEN
    delete from pool_aggregations where pool_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

--
--  pool trigger for event_aggregations.
--

CREATE OR REPLACE TRIGGER pool_aggregations_main 
    AFTER INSERT OR DELETE
    ON pools
    FOR EACH ROW
    EXECUTE PROCEDURE pool_aggregations_main();  