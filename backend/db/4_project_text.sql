-- public.project_text definition

-- Drop table

-- DROP TABLE public.project_text;

CREATE TABLE public.project_text (
	project_id int4 NOT NULL,
	lang_id int4 NOT NULL,
	title text NULL,
	description text NULL,
	short_description varchar(100) NULL,
	CONSTRAINT project_text_languages_fk FOREIGN KEY (lang_id) REFERENCES public.languages(id),
	CONSTRAINT project_text_project_fk FOREIGN KEY (project_id) REFERENCES public.project(id)
);