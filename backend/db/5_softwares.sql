-- public.softwares definition

-- Drop table

-- DROP TABLE public.softwares;

CREATE TABLE public.softwares (
	id serial4 NOT NULL,
	software text NOT NULL,
	icon text NOT NULL,
	type_icon text NOT NULL,
	color varchar(7) NULL,
	CONSTRAINT softwares_pkey PRIMARY KEY (id)
);