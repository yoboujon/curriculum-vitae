-- public.project_tags definition

-- Drop table

-- DROP TABLE public.project_tags;

CREATE TABLE public.project_tags (
	project_id int4 NOT NULL,
	skills_id int4 NOT NULL,
	CONSTRAINT project_tags_project_fk FOREIGN KEY (project_id) REFERENCES public.project(id),
	CONSTRAINT project_tags_skills_fk FOREIGN KEY (skills_id) REFERENCES public.skills(id)
);