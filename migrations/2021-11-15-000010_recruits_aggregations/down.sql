--
--  Drop Tenant Aggregations functions and triggers
--

-- job_aggregations_stages
DROP TRIGGER job_aggregations_stages ON job_stages;
DROP FUNCTION IF EXISTS job_aggregations_stages();
-- job_aggregations_sources
DROP TRIGGER job_aggregations_sources ON job_sources;
DROP FUNCTION IF EXISTS job_aggregations_sources();
-- job_aggregations_candidates
DROP TRIGGER job_aggregations_candidates ON job_candidates;
DROP FUNCTION IF EXISTS job_aggregations_candidates();
-- job_aggregations_events
DROP TRIGGER job_aggregations_events ON job_events;
DROP FUNCTION IF EXISTS job_aggregations_events();
-- job_aggregations_members
DROP TRIGGER job_aggregations_members ON job_members;
DROP FUNCTION IF EXISTS job_aggregations_members();

-- job_source_aggregations_candidates
DROP TRIGGER job_source_aggregations_candidates ON job_candidates;
DROP FUNCTION IF EXISTS job_source_aggregations_candidates();

-- job_stage_aggregations_candidates
DROP TRIGGER job_stage_aggregations_candidates ON job_candidates;
DROP FUNCTION IF EXISTS job_stage_aggregations_candidates();

-- job_stage_aggregations_events
DROP TRIGGER job_stage_aggregations_events ON job_events;
DROP FUNCTION IF EXISTS job_stage_aggregations_events();

-- job_offer_aggregations_variables
DROP TRIGGER job_offer_aggregations_variables ON job_offer_variables;
DROP FUNCTION IF EXISTS job_offer_aggregations_variables();


-- job_offer_aggregations_documents
DROP TRIGGER job_offer_aggregations_documents ON job_offer_documents;
DROP FUNCTION IF EXISTS job_offer_aggregations_documents();

-- job_candidate_aggregations_events
DROP TRIGGER job_candidate_aggregations_events ON job_events;
DROP FUNCTION IF EXISTS job_candidate_aggregations_events();

-- job_candidate_aggregations_offers
DROP TRIGGER job_candidate_aggregations_offers ON job_offers;
DROP FUNCTION IF EXISTS job_candidate_aggregations_offers();

-- job_event_aggregations_organizers
DROP TRIGGER job_event_aggregations_organizers ON job_event_organizers;
DROP FUNCTION IF EXISTS job_event_aggregations_organizers();

-- job_event_aggregations_candidates
DROP TRIGGER job_event_aggregations_candidates ON job_event_candidates;
DROP FUNCTION IF EXISTS job_event_aggregations_candidates();
