-- public.project definition

-- Drop table

-- DROP TABLE public.project;

CREATE TABLE public.project (
	id serial4 NOT NULL,
	date_done date NULL,
	title text NULL,
	description text NULL,
	github_link text NULL,
	id_skills serial4 NOT NULL,
	picture_name text NULL,
	type_project text NULL,
	CONSTRAINT project_pkey PRIMARY KEY (id),
	CONSTRAINT project_fk FOREIGN KEY (id_skills) REFERENCES public.skills(id)
);