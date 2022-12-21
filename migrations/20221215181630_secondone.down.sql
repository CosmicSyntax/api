-- Add down migration script here
BEGIN;

	drop table if exists demography cascade;

COMMIT;
