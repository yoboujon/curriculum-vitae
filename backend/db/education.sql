-- public.education definition

-- Drop table

-- DROP TABLE public.education;

CREATE TABLE public.education (
	id serial4 NOT NULL,
	start_year time NULL,
	end_year time NULL,
	school text NULL,
	speciality text NULL,
	school_location text NULL,
	school_options text NULL,
	CONSTRAINT education_pkey PRIMARY KEY (id)
);