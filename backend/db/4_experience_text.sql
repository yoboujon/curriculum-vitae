-- public.experience_text definition

-- Drop table

-- DROP TABLE public.experience_text;

CREATE TABLE public.experience_text (
	experience_id int4 NOT NULL,
	lang_id serial4 NOT NULL,
	job_position text NULL,
	job_description text NULL,
	enterprise_location text NULL,
	CONSTRAINT experience_text_experience_fk FOREIGN KEY (experience_id) REFERENCES public.experience(id),
	CONSTRAINT experience_text_languages_fk FOREIGN KEY (lang_id) REFERENCES public.languages(id)
);