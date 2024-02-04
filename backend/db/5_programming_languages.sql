-- public.programming_languages definition

-- Drop table

-- DROP TABLE public.programming_languages;

CREATE TABLE public.programming_languages (
	id serial4 NOT NULL,
	lang text NOT NULL,
	icon text NOT NULL,
	type_icon text NOT NULL,
	color varchar(7) NULL,
	CONSTRAINT programming_languages_pkey PRIMARY KEY (id)
);