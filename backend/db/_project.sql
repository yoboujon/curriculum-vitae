-- public.project definition

-- Drop table

-- DROP TABLE public.project;

CREATE TABLE public.project (
	id serial4 NOT NULL,
	date_done date NULL,
	title text NULL,
	description text NULL,
	github_link text NULL,
	info_id int4 NOT NULL DEFAULT nextval('project_id_skills_seq'::regclass),
	picture_name text NULL,
	type_project text NULL,
	CONSTRAINT project_pkey PRIMARY KEY (id),
	CONSTRAINT project_fk FOREIGN KEY (info_id) REFERENCES public.info(id)
);