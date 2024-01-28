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
use db::{Education, Experience, Info, Languages, ProgrammingLanguages, Project, Softwares, Tags, AllTags};

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
        .route("/info", get(info))
        .route("/education", get(education))
        .route("/experience", get(experience))
        .route("/skills/:id", get(skills))
        .route("/tags/:info_id/:project_id", get(tags))
        .route("/tags/:id", get(alltags))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    // Server
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    Ok(axum_server::bind(address)
        .serve(router.into_make_service())
        .await?)
}

macro_rules! _gather_data {
    ($data_type:ty, $sql_cmd:expr, $pool:expr) => {
        sqlx::query_as!($data_type, $sql_cmd).fetch_all($pool).await
    };
}

async fn info(State(pool): State<PgPool>) -> Json<Vec<Info>> {
    let datas = sqlx::query_as!(
        Info,
        "SELECT id, full_name, phone_number, email, softskills, interests, birth_year FROM public.info"
    )
    .fetch_all(&pool)
    .await.unwrap_or(vec![]);
    Json(datas)
}

async fn education(State(pool): State<PgPool>) -> Json<Vec<Education>> {
    let datas = sqlx::query_as!(Education, "SELECT * FROM public.education")
        .fetch_all(&pool)
        .await
        .unwrap_or(vec![]);
    Json(datas)
}

async fn experience(State(pool): State<PgPool>) -> Json<Vec<Experience>> {
    let datas = sqlx::query_as!(Experience, "SELECT * FROM public.experience")
        .fetch_all(&pool)
        .await
        .unwrap_or(vec![]);
    Json(datas)
}

async fn skills(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Json<(
    Vec<Project>,
    Vec<ProgrammingLanguages>,
    Vec<Softwares>,
    Vec<Languages>,
)> {
    let project = sqlx::query_as!(
        Project,
        "SELECT id, date_done, title, description, github_link, picture_name, type_project FROM public.project WHERE project.info_id = $1 ORDER BY date_done DESC",
        id
    )
    .fetch_all(&pool)
    .await.unwrap_or(vec![]);

    let programming_languages = sqlx::query_as!(
        ProgrammingLanguages,
        "SELECT lang, icon, type_icon, color FROM public.programming_languages WHERE programming_languages.info_id = $1 ORDER BY programming_languages.id",
        id
    )
    .fetch_all(&pool)
    .await.unwrap_or(vec![]);

    let softwares = sqlx::query_as!(
        Softwares,
        "SELECT software, icon, type_icon, color FROM public.softwares WHERE softwares.info_id = $1 ORDER BY softwares.id",
        id
    )
    .fetch_all(&pool)
    .await.unwrap_or(vec![]);

    let languages = sqlx::query_as!(
        Languages,
        "SELECT lang, icon_alpha, level FROM public.languages WHERE languages.info_id = $1 ORDER BY languages.id",
        id
    )
    .fetch_all(&pool)
    .await.unwrap_or(vec![]);

    Json((project, programming_languages, softwares, languages))
}

async fn tags(Path(ids): Path<(i32, i32)>, State(pool): State<PgPool>) -> Json<Vec<Tags>> {
    let (info_id, project_id) = ids;
    let datas = sqlx::query_as!(
        Tags,
        "SELECT lang, icon, type_icon, color
        FROM public.programming_languages pl
        JOIN public.project_tags pt ON pl.id = pt.programming_languages_id
        WHERE pt.info_id = $1 and pt.project_id = $2
        
        UNION ALL
        
        SELECT software, icon, type_icon, color
        FROM public.softwares s
        JOIN public.project_tags pt ON s.id = pt.softwares_id
        WHERE pt.info_id = $1 and pt.project_id = $2;
        
    ",
        info_id,
        project_id
    )
    .fetch_all(&pool)
    .await
    .unwrap_or(vec![]);
    Json(datas)
}

async fn alltags(Path(info_id): Path<i32>, State(pool): State<PgPool>) -> Json<Vec<AllTags>> {
    let datas = sqlx::query_as!(
        AllTags,
        "SELECT project_id, lang, icon, type_icon, color
        FROM public.programming_languages pl
        JOIN public.project_tags pt ON pl.id = pt.programming_languages_id
        WHERE pt.info_id = $1
        
        UNION ALL
        
        SELECT project_id, software, icon, type_icon, color
        FROM public.softwares s
        JOIN public.project_tags pt ON s.id = pt.softwares_id
        WHERE pt.info_id = $1
        ORDER BY project_id;
        
    ",
        info_id
    )
    .fetch_all(&pool)
    .await
    .unwrap_or(vec![]);
    Json(datas)
}
