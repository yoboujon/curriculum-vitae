-- public.experience definition

-- Drop table

-- DROP TABLE public.experience;

CREATE TABLE public.experience (
	id serial4 NOT NULL,
	job_position text NULL,
	job_description text NULL,
	enterprise text NULL,
	enterprise_location text NULL,
	start_year date NULL,
	end_year date NULL,
	CONSTRAINT experience_pkey PRIMARY KEY (id)
);