-- Add down migration script here
BEGIN;

	drop table if exists customers cascade;
	drop table if exists entries cascade;

COMMIT;
