-- public.skills definition

-- Drop table

-- DROP TABLE public.skills;

CREATE TABLE public.skills (
	id serial4 NOT NULL,
	programming_lang text NULL,
	software text NULL,
	languages text NULL,
	CONSTRAINT skills_pkey PRIMARY KEY (id)
);