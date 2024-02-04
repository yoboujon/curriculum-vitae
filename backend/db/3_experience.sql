-- public.experience definition

-- Drop table

-- DROP TABLE public.experience;

CREATE TABLE public.experience (
	id serial4 NOT NULL,
	enterprise text NULL,
	start_year date NULL,
	end_year date NULL,
	picture_url text NULL,
	CONSTRAINT experience_pkey PRIMARY KEY (id)
);