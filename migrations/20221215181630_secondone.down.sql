-- Add down migration script here
BEGIN;

	drop table if exists information cascade;

COMMIT;
