-- public.education_text definition

-- Drop table

-- DROP TABLE public.education_text;

CREATE TABLE public.education_text (
	education_id serial4 NOT NULL,
	lang_id serial4 NOT NULL,
	speciality text NULL,
	school_location text NULL,
	school_options text NULL,
	CONSTRAINT education_text_education_fk FOREIGN KEY (education_id) REFERENCES public.education(id),
	CONSTRAINT education_text_languages_fk FOREIGN KEY (lang_id) REFERENCES public.languages(id)
);