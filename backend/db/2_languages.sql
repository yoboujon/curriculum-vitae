-- public.languages definition

-- Drop table

-- DROP TABLE public.languages;

CREATE TABLE public.languages (
	id serial4 NOT NULL,
	lang text NOT NULL,
	icon_alpha varchar(3) NOT NULL,
	"level" text NOT NULL,
	url_name varchar(2) NOT NULL,
	CONSTRAINT languages_pkey PRIMARY KEY (id)
);