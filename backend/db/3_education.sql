-- public.education definition

-- Drop table

-- DROP TABLE public.education;

CREATE TABLE public.education (
	id serial4 NOT NULL,
	start_year date NULL,
	end_year date NULL,
	school text NULL,
	picture_url text NULL,
	CONSTRAINT education_pkey PRIMARY KEY (id)
);