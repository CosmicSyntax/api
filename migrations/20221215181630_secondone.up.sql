-- Add up migration script here
BEGIN;


	CREATE TABLE IF NOT EXISTS public.information (
		id integer NOT NULL,
		username text NOT NULL,
		pw bytea NOT NULL,
		created_at timestamp with time zone DEFAULT now() NOT NULL,
		updated_at timestamp with time zone DEFAULT now() NOT NULL,
		FOREIGN KEY (id) REFERENCES customers(id)
	);

COMMIT;
