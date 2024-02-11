-- public.categories definition

-- Drop table

-- DROP TABLE public.categories;

CREATE TABLE public.categories (
	id serial4 NOT NULL,
	"name" text NOT NULL,
	icon text NOT NULL,
	type_icon text NOT NULL,
	CONSTRAINT categories_pk PRIMARY KEY (id)
);