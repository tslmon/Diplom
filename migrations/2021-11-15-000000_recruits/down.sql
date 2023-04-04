
--
--  Drop recruit Management.
--

DROP TABLE IF EXISTS jobs CASCADE;
DROP TABLE IF EXISTS job_sources CASCADE;
DROP TABLE IF EXISTS job_stages CASCADE;
DROP TABLE IF EXISTS job_candidates CASCADE;
DROP TABLE IF EXISTS job_offers CASCADE;
DROP TABLE IF EXISTS job_events CASCADE;
DROP TABLE IF EXISTS job_members CASCADE;
DROP TABLE IF EXISTS job_offer_variables CASCADE;
DROP TABLE IF EXISTS job_offer_documents CASCADE;
DROP TABLE IF EXISTS job_event_candidates CASCADE;
DROP TABLE IF EXISTS job_event_organizers CASCADE;

DROP TABLE IF EXISTS pools CASCADE;
DROP TABLE IF EXISTS pool_members CASCADE;
DROP TABLE IF EXISTS pool_candidates CASCADE;

DROP TABLE IF EXISTS job_aggregations CASCADE;
DROP TABLE IF EXISTS job_source_aggregations CASCADE;
DROP TABLE IF EXISTS job_stage_aggregations CASCADE;
DROP TABLE IF EXISTS job_offer_aggregations CASCADE;
DROP TABLE IF EXISTS job_candidate_aggregations CASCADE;
DROP TABLE IF EXISTS job_event_aggregations CASCADE;

DROP TABLE IF EXISTS pool_aggregations CASCADE;

DROP FUNCTION IF EXISTS job_aggregations_main CASCADE;
DROP FUNCTION IF EXISTS job_source_aggregations_main CASCADE;
DROP FUNCTION IF EXISTS job_stage_aggregations_main CASCADE;
DROP FUNCTION IF EXISTS job_offer_aggregations_main CASCADE;
DROP FUNCTION IF EXISTS job_candidate_aggregations_main CASCADE;
DROP FUNCTION IF EXISTS job_event_aggregations_main CASCADE;

DROP FUNCTION IF EXISTS pool_aggregations_main CASCADE;