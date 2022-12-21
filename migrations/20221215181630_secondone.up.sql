-- Add up migration script here
BEGIN;


	CREATE TABLE IF NOT EXISTS public.demography (
		id integer NOT NULL,
		content text,
		created_at timestamp with time zone DEFAULT now() NOT NULL,
		updated_at timestamp with time zone DEFAULT now() NOT NULL,
		FOREIGN KEY (id) REFERENCES customers(id)
	);

COMMIT;
