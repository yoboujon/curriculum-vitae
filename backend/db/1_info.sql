-- public.info definition

-- Drop table

-- DROP TABLE public.info;

CREATE TABLE public.info (
	birth_year date NULL,
	id serial4 NOT NULL,
	full_name text NULL,
	phone_number varchar NULL,
	email varchar NULL,
	softskills text NULL,
	interests text NULL,
	CONSTRAINT info_pkey PRIMARY KEY (id)
);