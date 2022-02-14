BEGIN;

	CREATE TABLE IF NOT EXISTS public.customers (
		uuid uuid NOT NULL,
		created_at timestamp with time zone DEFAULT now() NOT NULL,
		updated_at timestamp with time zone DEFAULT now() NOT NULL,
		id integer NOT NULL
	);

	CREATE SEQUENCE IF NOT EXISTS public.customers_id_seq
		START WITH 1
		INCREMENT BY 1
		NO MINVALUE
		NO MAXVALUE
		CACHE 1;

	ALTER SEQUENCE public.customers_id_seq OWNED BY public.customers.id;

COMMIT;
