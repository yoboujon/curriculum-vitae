-- public.softwares definition

-- Drop table

-- DROP TABLE public.softwares;

CREATE TABLE public.softwares (
	id serial4 NOT NULL,
	info_id int4 NOT NULL DEFAULT nextval('softwares_skills_id_seq'::regclass),
	software text NOT NULL,
	icon text NOT NULL,
	type_icon text NOT NULL,
	color varchar(7) NULL,
	CONSTRAINT softwares_pkey PRIMARY KEY (id)
);