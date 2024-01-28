-- public.project_tags definition

-- Drop table

-- DROP TABLE public.project_tags;

CREATE TABLE public.project_tags (
	project_id int4 NOT NULL,
	programming_languages_id int4 NULL,
	softwares_id int4 NULL,
	CONSTRAINT project_tags_project_fk FOREIGN KEY (project_id) REFERENCES public.project(id),
	CONSTRAINT project_tags_programming_languages_fk FOREIGN KEY (programming_languages_id) REFERENCES public.programming_languages(id),
	CONSTRAINT project_tags_softwares_fk FOREIGN KEY (softwares_id) REFERENCES public.softwares(id)
);