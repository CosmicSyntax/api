-- Add up migration script here
BEGIN;

	CREATE TABLE IF NOT EXISTS public.customers (
		id integer NOT NULL GENERATED ALWAYS AS IDENTITY,
		uuid uuid NOT NULL,
		created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
		updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
		PRIMARY KEY (id)
	);

	CREATE INDEX ON public.customers USING btree(uuid);

	/* CREATE SEQUENCE IF NOT EXISTS public.customers_id_seq
		START WITH 2
		INCREMENT BY 2
		NO MINVALUE
		NO MAXVALUE
		CACHE 1;
	ALTER TABLE ONLY public.customers ALTER COLUMN id SET DEFAULT nextval('public.customers_id_seq'); */

	CREATE TABLE IF NOT EXISTS public.entries (
		id integer NOT NULL,
		content text,
		created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
		updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
		FOREIGN KEY (id) REFERENCES customers(id)
	);

COMMIT;
