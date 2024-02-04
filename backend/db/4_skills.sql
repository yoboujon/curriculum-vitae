-- public.skills definition

-- Drop table

-- DROP TABLE public.skills;

CREATE TABLE public.skills (
	softskills text NULL,
	interests text NULL,
	lang_id serial4 NOT NULL,
	CONSTRAINT skills_languages_fk FOREIGN KEY (lang_id) REFERENCES public.languages(id)
);