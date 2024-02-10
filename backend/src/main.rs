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
    AllTags, Education, Experience, Info, LangId, Languages, ProgrammingLanguages, Project,
    SimpleProject, Softwares, Tags,
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
        .route("/hard_skills", get(hard_skills))
        .route("/tags/:project_id", get(tags))
        .route("/tags", get(alltags))
        .route(
            "/getproject_programming/:programming_id/:lang_id",
            get(getproject_programming),
        )
        .route(
            "/getproject_software/:software_id/:lang_id",
            get(getproject_software),
        )
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
        (SELECT title FROM public.skills WHERE lang_id = $1 LIMIT 1) AS title,
        (SELECT softskills FROM public.skills WHERE lang_id = $1 LIMIT 1) AS softskills,
        (SELECT interests FROM public.skills WHERE lang_id = $1 LIMIT 1) AS interests,
        (SELECT description FROM public.skills WHERE lang_id = $1 LIMIT 1) AS description;",
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

async fn hard_skills(
    State(pool): State<PgPool>,
) -> Json<(Vec<ProgrammingLanguages>, Vec<Softwares>, Vec<Languages>)> {
    let programming_languages = sqlx::query_as!(
        ProgrammingLanguages,
        "SELECT
        pl.lang,
        pl.icon,
        pl.type_icon,
        pl.color
    FROM public.programming_languages pl
    ORDER BY pl.id;"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or(vec![]);

    let softwares = sqlx::query_as!(
        Softwares,
        "SELECT
        s.software,
        s.icon,
        s.type_icon,
        s.color
    FROM public.softwares s
    ORDER BY s.id;"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or(vec![]);

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
    Json((programming_languages, softwares, languages))
}

async fn tags(Path(project_id): Path<i32>, State(pool): State<PgPool>) -> Json<Vec<Tags>> {
    let datas = sqlx::query_as!(
        Tags,
        "SELECT lang, icon, type_icon, color
        FROM public.programming_languages pl
        JOIN public.project_tags pt ON pl.id = pt.programming_languages_id
        WHERE pt.project_id = $1
        
        UNION ALL
        
        SELECT software, icon, type_icon, color
        FROM public.softwares s
        JOIN public.project_tags pt ON s.id = pt.softwares_id
        WHERE pt.project_id = $1;",
        project_id
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    Json(datas)
}

async fn alltags(State(pool): State<PgPool>) -> Json<Vec<AllTags>> {
    let datas = sqlx::query_as!(
        AllTags,
        "SELECT project_id, lang, icon, type_icon, color
        FROM public.programming_languages pl
        JOIN public.project_tags pt ON pl.id = pt.programming_languages_id
        
        UNION ALL
        
        SELECT project_id, software, icon, type_icon, color
        FROM public.softwares s
        JOIN public.project_tags pt ON s.id = pt.softwares_id
        ORDER BY project_id;"
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    Json(datas)
}

async fn getproject_programming(
    Path(ids): Path<(i32,i32)>,
    State(pool): State<PgPool>,
) -> Json<Vec<SimpleProject>> {
    let (programming_id, lang_id) = ids;
    let datas = sqlx::query_as!(
        SimpleProject,
        " SELECT
        pt.project_id,
        pt.title
    FROM public.project_text pt
    JOIN public.project_tags pta ON (pt.project_id = pta.project_id) AND (pt.lang_id = $1) 
    WHERE pta.programming_languages_id = $2
    ORDER BY pt.project_id;",
        lang_id,
        programming_id
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    Json(datas)
}

async fn getproject_software(
    Path(ids): Path<(i32,i32)>,
    State(pool): State<PgPool>,
) -> Json<Vec<SimpleProject>> {
    let (software_id, lang_id) = ids;
    let datas = sqlx::query_as!(
        SimpleProject,
        " SELECT
        pt.project_id,
        pt.title
    FROM public.project_text pt
    JOIN public.project_tags pta ON (pt.project_id = pta.project_id) AND (pt.lang_id = $1) 
    WHERE pta.softwares_id = $2
    ORDER BY pt.project_id;",
        lang_id,
        software_id
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    Json(datas)
}
