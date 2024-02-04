-- public.project definition

-- Drop table

-- DROP TABLE public.project;

CREATE TABLE public.project (
	id serial4 NOT NULL,
	date_done date NULL,
	github_link text NULL,
	picture_name text NULL,
	type_project text NULL,
	report_link text NULL,
	archive_link text NULL,
	app_link text NULL,
	CONSTRAINT project_pk PRIMARY KEY (id)
);