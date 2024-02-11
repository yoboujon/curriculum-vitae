-- public.categories_text definition

-- Drop table

-- DROP TABLE public.categories_text;

CREATE TABLE public.categories_text (
	category_id serial4 NOT NULL,
	lang_id serial4 NOT NULL,
	"name" text NOT NULL,
	CONSTRAINT categories_text_categories_fk FOREIGN KEY (category_id) REFERENCES public.categories(id),
	CONSTRAINT categories_text_languages_fk FOREIGN KEY (lang_id) REFERENCES public.languages(id)
);