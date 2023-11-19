use axum::{
    extract::{Path, State},
    routing::{get, post},
    Form, Json, Router,
};
use axum_error::Result;
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod db;
use db::{Education, Experience, Info, Languages, Project, ProgrammingLanguages, Softwares};

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
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    // Server
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    Ok(axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await?)
}

macro_rules! gather_data {
    ($data_type:ty, $sql_cmd:expr, $pool:expr) => {
        sqlx::query_as!($data_type, $sql_cmd).fetch_all($pool).await
    };
}

async fn info(State(pool): State<PgPool>) -> Result<Json<Vec<Info>>> {
    let datas = sqlx::query_as!(
        Info,
        "SELECT id, full_name, phone_number, email, softskills, interests, birth_year FROM public.info"
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(datas))
}

async fn education(State(pool): State<PgPool>) -> Result<Json<Vec<Education>>> {
    let datas = sqlx::query_as!(Education, "SELECT * FROM public.education")
        .fetch_all(&pool)
        .await?;
    Ok(Json(datas))
}

async fn experience(State(pool): State<PgPool>) -> Result<Json<Vec<Experience>>> {
    let datas = sqlx::query_as!(Experience, "SELECT * FROM public.experience")
        .fetch_all(&pool)
        .await?;
    Ok(Json(datas))
}

async fn skills(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<(Vec<Project>,Vec<ProgrammingLanguages>,Vec<Softwares>,Vec<Languages>)>> {
    let project = sqlx::query_as!(
        Project,
        "SELECT date_done, title, description, github_link, picture_name, type_project FROM public.project WHERE project.info_id = $1 ORDER BY date_done DESC",
        id
    )
    .fetch_all(&pool)
    .await?;
    
    let programming_languages = sqlx::query_as!(
        ProgrammingLanguages,
        "SELECT lang, icon, type_icon, color FROM public.programming_languages WHERE programming_languages.info_id = $1 ORDER BY programming_languages.id",
        id
    )
    .fetch_all(&pool)
    .await?;

    let softwares = sqlx::query_as!(
        Softwares,
        "SELECT software, icon, type_icon, color FROM public.softwares WHERE softwares.info_id = $1 ORDER BY softwares.id",
        id
    )
    .fetch_all(&pool)
    .await?;

    let languages = sqlx::query_as!(
        Languages,
        "SELECT lang, icon_alpha, level FROM public.languages WHERE languages.info_id = $1 ORDER BY languages.id",
        id
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json((project,programming_languages,softwares,languages)))
}
