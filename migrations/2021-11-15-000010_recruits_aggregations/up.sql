
--
--  stages
--
CREATE FUNCTION job_aggregations_stages()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    update job_aggregations 
    set stages = stages + 1
    where job_id = NEW.id;
  ELSIF (TG_OP = 'DELETE') THEN
    update job_aggregations sa
    set stages = stages - 1
    where job_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

CREATE TRIGGER job_aggregations_stages
    AFTER INSERT OR DELETE
    ON job_stages
    FOR EACH ROW
    EXECUTE PROCEDURE job_aggregations_stages();

--
--  sources
--
CREATE FUNCTION job_aggregations_sources()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    update job_aggregations 
    set sources = sources + 1
    where job_id = NEW.id;
  ELSIF (TG_OP = 'DELETE') THEN
    update job_aggregations sa
    set sources = sources - 1
    where job_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

CREATE TRIGGER job_aggregations_sources
    AFTER INSERT OR DELETE
    ON job_sources
    FOR EACH ROW
    EXECUTE PROCEDURE job_aggregations_sources();

--
--  candidates
--
CREATE FUNCTION job_aggregations_candidates()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    update job_aggregations 
    set candidates = candidates + 1
    where job_id = NEW.id;
  ELSIF (TG_OP = 'DELETE') THEN
    update job_aggregations sa
    set candidates = candidates - 1
    where job_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

CREATE TRIGGER job_aggregations_candidates
    AFTER INSERT OR DELETE
    ON job_candidates
    FOR EACH ROW
    EXECUTE PROCEDURE job_aggregations_candidates();

--
--  events
--
CREATE FUNCTION job_aggregations_events()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    update job_aggregations 
    set events = events + 1
    where job_id = NEW.id;
  ELSIF (TG_OP = 'DELETE') THEN
    update job_aggregations sa
    set events = events - 1
    where job_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

CREATE TRIGGER job_aggregations_events
    AFTER INSERT OR DELETE
    ON job_events
    FOR EACH ROW
    EXECUTE PROCEDURE job_aggregations_events();

--
--  members
--
CREATE FUNCTION job_aggregations_members()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    update job_aggregations 
    set members = members + 1
    where job_id = NEW.id;
  ELSIF (TG_OP = 'DELETE') THEN
    update job_aggregations sa
    set members = members - 1
    where job_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

CREATE TRIGGER job_aggregations_members
    AFTER INSERT OR DELETE
    ON job_members
    FOR EACH ROW
    EXECUTE PROCEDURE job_aggregations_members();

--
--  candidates
--
CREATE FUNCTION job_source_aggregations_candidates()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    update job_source_aggregations 
    set candidates = candidates + 1
    where source_id = NEW.id;
  ELSIF (TG_OP = 'DELETE') THEN
    update job_source_aggregations sa
    set candidates = candidates - 1
    where source_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

CREATE TRIGGER job_source_aggregations_candidates
    AFTER INSERT OR DELETE
    ON job_candidates
    FOR EACH ROW
    EXECUTE PROCEDURE job_source_aggregations_candidates();

--
--  candidates
--
CREATE FUNCTION job_stage_aggregations_candidates()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    update job_stage_aggregations 
    set candidates = candidates + 1
    where stage_id = NEW.id;
  ELSIF (TG_OP = 'DELETE') THEN
    update job_stage_aggregations sa
    set candidates = candidates - 1
    where stage_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

CREATE TRIGGER job_stage_aggregations_candidates
    AFTER INSERT OR DELETE
    ON job_candidates
    FOR EACH ROW
    EXECUTE PROCEDURE job_stage_aggregations_candidates();

--
--  events
--
CREATE FUNCTION job_stage_aggregations_events()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    update job_stage_aggregations 
    set events = events + 1
    where stage_id = NEW.id;
  ELSIF (TG_OP = 'DELETE') THEN
    update job_stage_aggregations sa
    set events = events - 1
    where stage_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

CREATE TRIGGER job_stage_aggregations_events
    AFTER INSERT OR DELETE
    ON job_events
    FOR EACH ROW
    EXECUTE PROCEDURE job_stage_aggregations_events();

--
--  variables
--
CREATE FUNCTION job_offer_aggregations_variables()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    update job_offer_aggregations 
    set variables = variables + 1
    where offer_id = NEW.id;
  ELSIF (TG_OP = 'DELETE') THEN
    update job_offer_aggregations sa
    set variables = variables - 1
    where offer_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

CREATE TRIGGER job_offer_aggregations_variables
    AFTER INSERT OR DELETE
    ON job_offer_variables
    FOR EACH ROW
    EXECUTE PROCEDURE job_offer_aggregations_variables();

--
--  documents
--
CREATE FUNCTION job_offer_aggregations_documents()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    update job_offer_aggregations 
    set documents = documents + 1
    where offer_id = NEW.id;
  ELSIF (TG_OP = 'DELETE') THEN
    update job_offer_aggregations sa
    set documents = documents - 1
    where offer_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

CREATE TRIGGER job_offer_aggregations_documents
    AFTER INSERT OR DELETE
    ON job_offer_documents
    FOR EACH ROW
    EXECUTE PROCEDURE job_offer_aggregations_documents();

--
--  events
--
CREATE FUNCTION job_candidate_aggregations_events()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    update job_candidate_aggregations 
    set events = events + 1
    where candidate_id = NEW.id;
  ELSIF (TG_OP = 'DELETE') THEN
    update job_candidate_aggregations sa
    set events = events - 1
    where candidate_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

CREATE TRIGGER job_candidate_aggregations_events
    AFTER INSERT OR DELETE
    ON job_events
    FOR EACH ROW
    EXECUTE PROCEDURE job_candidate_aggregations_events();    

--
--  offers
--
CREATE FUNCTION job_candidate_aggregations_offers()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    update job_candidate_aggregations 
    set offers = offers + 1
    where candidate_id = NEW.id;
  ELSIF (TG_OP = 'DELETE') THEN
    update job_candidate_aggregations sa
    set offers = offers - 1
    where candidate_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

CREATE TRIGGER job_candidate_aggregations_offers
    AFTER INSERT OR DELETE
    ON job_offers
    FOR EACH ROW
    EXECUTE PROCEDURE job_candidate_aggregations_offers();

--
--  organizers
--
CREATE FUNCTION job_event_aggregations_organizers()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    update job_event_aggregations 
    set organizers = organizers + 1
    where event_id = NEW.id;
  ELSIF (TG_OP = 'DELETE') THEN
    update job_event_aggregations sa
    set organizers = organizers - 1
    where event_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

CREATE TRIGGER job_event_aggregations_organizers
    AFTER INSERT OR DELETE
    ON job_event_organizers
    FOR EACH ROW
    EXECUTE PROCEDURE job_event_aggregations_organizers();

--
--  candidates
--
CREATE FUNCTION job_event_aggregations_candidates()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    update job_event_aggregations 
    set candidates = candidates + 1
    where event_id = NEW.id;
  ELSIF (TG_OP = 'DELETE') THEN
    update job_event_aggregations sa
    set candidates = candidates - 1
    where event_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

CREATE TRIGGER job_event_aggregations_candidates
    AFTER INSERT OR DELETE
    ON job_event_candidates
    FOR EACH ROW
    EXECUTE PROCEDURE job_event_aggregations_candidates();    