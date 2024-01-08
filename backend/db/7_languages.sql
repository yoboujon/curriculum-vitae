-- public.languages definition

-- Drop table

-- DROP TABLE public.languages;

CREATE TABLE public.languages (
	id serial4 NOT NULL,
	info_id int4 NOT NULL,
	lang text NOT NULL,
	icon_alpha varchar(3) NOT NULL,
	"level" text NOT NULL,
	CONSTRAINT languages_pkey PRIMARY KEY (id),
	CONSTRAINT languages_fk FOREIGN KEY (info_id) REFERENCES public.info(id)
);