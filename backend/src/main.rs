use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use axum_error::Result;
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod db;
use db::{
    AllSkills, Categories, CategoriesText, Education, Experience, Id, Info, LangId, Languages,
    Project, SimpleProject, Skills,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Environnement table + pool connection
    dotenv().ok();
    let url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    // Creating a router
    let router = Router::new()
        .route("/get_lang_id/:lang", get(get_lang_id))
        .route("/info/:lang_id", get(info))
        .route("/education/:lang_id", get(education))
        .route("/experience/:lang_id", get(experience))
        .route("/project/:lang_id", get(projects))
        .route("/categories/:lang_id", get(categories_text))
        .route("/hard_skills", get(hard_skills))
        .route("/tags/:project_id", get(tags))
        .route("/tags", get(alltags))
        .route("/getproject_programming/:lang_id", get(getproject_skills))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    // Server
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    Ok(axum_server::bind(address)
        .serve(router.into_make_service())
        .await?)
}

async fn get_lang_id(Path(lang): Path<String>, State(pool): State<PgPool>) -> Json<LangId> {
    let datas = sqlx::query_as!(
        LangId,
        "SELECT id
    FROM public.languages l
    WHERE l.url_name = $1;",
        lang
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    if datas.len() > 0 {
        Json(datas[0])
    } else {
        Json(LangId { id: Option::None })
    }
}

async fn info(Path(lang_id): Path<i32>, State(pool): State<PgPool>) -> Json<Vec<Info>> {
    let datas = sqlx::query_as!(
        Info,
        "SELECT
        (SELECT full_name FROM public.info) AS full_name,
        (SELECT phone_number FROM public.info LIMIT 1) AS phone_number,
        (SELECT email FROM public.info LIMIT 1) AS email,
        (SELECT birth_year FROM public.info LIMIT 1) AS birth_year,
        (SELECT profile_pic FROM public.info LIMIT 1) AS profile_pic,
        (SELECT title FROM public.info_text WHERE lang_id = $1 LIMIT 1) AS title,
        (SELECT softskills FROM public.info_text WHERE lang_id = $1 LIMIT 1) AS softskills,
        (SELECT interests FROM public.info_text WHERE lang_id = $1 LIMIT 1) AS interests,
        (SELECT description FROM public.info_text WHERE lang_id = $1 LIMIT 1) AS description,
        (SELECT pdf_url FROM public.info_text WHERE lang_id = $1 LIMIT 1) AS pdf_url;",
        lang_id
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    Json(datas)
}

async fn education(Path(lang_id): Path<i32>, State(pool): State<PgPool>) -> Json<Vec<Education>> {
    let datas = sqlx::query_as!(
        Education,
        "SELECT
    e.school,
    e.start_year,
    e.end_year,
    e.picture_url,
    MAX(CASE WHEN et.lang_id = $1 THEN et.speciality END) AS speciality,
    MAX(CASE WHEN et.lang_id = $1 THEN et.school_options END) AS school_options,
    MAX(CASE WHEN et.lang_id = $1 THEN et.school_location END) AS school_location
FROM public.education e
JOIN public.education_text et ON e.id = et.education_id
GROUP BY e.school, e.start_year, e.end_year, e.picture_url
ORDER BY e.start_year ASC;
",
        lang_id
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    Json(datas)
}

async fn experience(Path(lang_id): Path<i32>, State(pool): State<PgPool>) -> Json<Vec<Experience>> {
    let datas = sqlx::query_as!(
        Experience,
        "SELECT
    e.enterprise,
    e.start_year,
    e.end_year,
    e.picture_url,
    MAX(CASE WHEN et.lang_id = $1 THEN et.job_position END) AS job_position,
    MAX(CASE WHEN et.lang_id = $1 THEN et.job_description END) AS job_description,
    MAX(CASE WHEN et.lang_id = $1 THEN et.enterprise_location END) AS enterprise_location
FROM public.experience e
JOIN public.experience_text et ON e.id = et.experience_id
GROUP BY e.enterprise, e.start_year, e.end_year, e.picture_url
ORDER BY e.start_year ASC;
",
        lang_id
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    Json(datas)
}

async fn projects(Path(lang_id): Path<i32>, State(pool): State<PgPool>) -> Json<Vec<Project>> {
    let datas = sqlx::query_as!(
        Project,
        "SELECT
    p.id,
    p.date_done,
    p.picture_name,
    p.type_project,
    p.github_link,
    p.report_link,
    p.archive_link,
    p.app_link,
    MAX(CASE WHEN pt.lang_id = $1 THEN pt.title END) AS title,
    MAX(CASE WHEN pt.lang_id = $1 THEN pt.description END) AS description,
    MAX(CASE WHEN pt.lang_id = $1 THEN pt.short_description END) AS short_description
FROM public.project p
JOIN public.project_text pt ON p.id = pt.project_id
GROUP BY p.id, p.date_done, p.picture_name, p.type_project, p.github_link, p.report_link, p.archive_link, p.app_link 
ORDER BY p.date_done DESC;",
        lang_id
    )
    .fetch_all(&pool)
    .await.unwrap();
    Json(datas)
}

async fn categories_text(
    Path(lang_id): Path<i32>,
    State(pool): State<PgPool>,
) -> Json<Vec<CategoriesText>> {
    let categories = sqlx::query_as!(
        Categories,
        "select
        c.id,
        c.name
    from
        public.categories c
    order by
        c.id;"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let mut categories_text: Vec<CategoriesText> = Vec::new();

    for c in categories {
        let temp_category = sqlx::query_as!(
            CategoriesText,
            "SELECT
            (SELECT icon FROM public.categories where id = $1 LIMIT 1) AS icon,
            (SELECT type_icon FROM public.categories where id = $1 LIMIT 1) AS type_icon,
            (SELECT name FROM public.categories_text WHERE category_id = $1 and lang_id = $2 LIMIT 1) AS name;"
            ,c.id.unwrap(),lang_id
        ).fetch_all(&pool)
        .await
        .unwrap();
        categories_text.push(temp_category[0].clone());
    }

    Json(categories_text)
}

async fn hard_skills(State(pool): State<PgPool>) -> Json<(Vec<Languages>, Vec<Vec<Skills>>)> {
    let languages = sqlx::query_as!(
        Languages,
        "SELECT
        l.lang,
        l.icon_alpha,
        l.level,
        l.url_name
    FROM public.languages l
    ORDER BY l.id;"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let categories = sqlx::query_as!(
        Categories,
        "select
        c.id,
        c.name
    from
        public.categories c
    order by
        c.id;"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let mut skills: Vec<Vec<Skills>> = Vec::new();

    for c in categories {
        let skill_list = sqlx::query_as!(
            Skills,
            "select
            s.id,
            s.skill,
            s.icon,
            s.type_icon,
            s.color,
            s.is_shown
        from
            public.skills s
        where
            s.category_id = $1
        order by
            s.id;",
            c.id.unwrap()
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        skills.push(skill_list);
    }

    Json((languages, skills))
}

async fn tags(Path(project_id): Path<i32>, State(pool): State<PgPool>) -> Json<Vec<Skills>> {
    let datas = sqlx::query_as!(
        Skills,
        "select
        s.id,
        s.skill,
        s.icon,
        s.type_icon,
        s.color,
        s.is_shown
    from
        public.skills s
    join public.project_tags pt on
        s.id = pt.skills_id
    where
        pt.project_id = $1",
        project_id
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    Json(datas)
}

async fn alltags(State(pool): State<PgPool>) -> Json<Vec<AllSkills>> {
    let datas = sqlx::query_as!(
        AllSkills,
        "select
        pt.project_id,
        s.skill,
        s.icon,
        s.type_icon,
        s.color,
        s.is_shown
    from
        public.skills s
    join public.project_tags pt on
        s.id = pt.skills_id
    order by
        pt.project_id;"
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    Json(datas)
}

async fn getproject_skills(
    Path(lang_id): Path<i32>,
    State(pool): State<PgPool>,
) -> Json<Vec<Vec<SimpleProject>>> {
    let project_ids = sqlx::query_as!(
        Id,
        "select
        s.id
    from
        public.skills s
    order by
        s.id;"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let mut project_skills: Vec<Vec<SimpleProject>> = Vec::new();

    for p in project_ids {
        project_skills.push(
            sqlx::query_as!(
                SimpleProject,
                "select
            pt.project_id,
            pt.title
        from
            public.project_text pt
        join public.project_tags pta on
            (pt.project_id = pta.project_id)
            and (pt.lang_id = $1)
        where
            pta.skills_id = $2
        order by
            pt.project_id;",
                lang_id,
                p.id.unwrap(),
            )
            .fetch_all(&pool)
            .await
            .unwrap(),
        );
    }

    Json(project_skills)
}
