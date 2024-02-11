-- public.skills definition

-- Drop table

-- DROP TABLE public.skills;

CREATE TABLE public.skills (
	id serial4 NOT NULL,
	category_id int4 DEFAULT nextval('skills_category_seq'::regclass) NOT NULL,
	skill text NULL,
	icon text NULL,
	type_icon text NULL,
	color varchar(7) NULL,
	is_shown bool NULL,
	CONSTRAINT skills_pk PRIMARY KEY (id),
	CONSTRAINT skills_categories_fk FOREIGN KEY (category_id) REFERENCES public.categories(id)
);