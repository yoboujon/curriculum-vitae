-- public.programming_languages definition

-- Drop table

-- DROP TABLE public.programming_languages;

CREATE TABLE public.programming_languages (
	id serial4 NOT NULL,
	info_id int4 NOT NULL DEFAULT nextval('programming_languages_skills_id_seq'::regclass),
	lang text NOT NULL,
	icon text NOT NULL,
	type_icon text NOT NULL,
	color varchar(7) NULL,
	CONSTRAINT programming_languages_pkey PRIMARY KEY (id),
	CONSTRAINT programming_languages_fk FOREIGN KEY (info_id) REFERENCES public.info(id)
);