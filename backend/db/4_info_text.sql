-- public.skills definition

-- Drop table

-- DROP TABLE public.skills;

CREATE TABLE public.info_text (
	softskills text NULL,
	interests text NULL,
	lang_id int4 NOT NULL,
	title text NULL,
	description text NULL,
	CONSTRAINT skills_languages_fk FOREIGN KEY (lang_id) REFERENCES public.languages(id)
);;