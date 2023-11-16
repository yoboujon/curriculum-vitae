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
use db::{Education, Experience, Info, Project, Skills};

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
        .route("/skills", get(skills))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    // Server
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    Ok(axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await?)
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
    let datas = sqlx::query_as!(
        Education,
        "SELECT * FROM public.education"
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(datas))
}

async fn experience(State(pool): State<PgPool>) -> Result<Json<Vec<Experience>>> {
    let datas = sqlx::query_as!(
        Experience,
        "SELECT * FROM public.experience"
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(datas))
}

async fn skills(State(pool): State<PgPool>) -> Result<Json<(Vec<Project>, Vec<Skills>)>> {
    // Gathering skills
    let skills = sqlx::query_as!(
        Skills,
        "SELECT id, programming_lang, software, languages FROM public.skills"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    // Gathering Projects
    let projects = sqlx::query_as!(
        Project,
        "SELECT id, date_done, title, description, github_link, id_skills FROM public.project"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    Ok(Json((projects,skills)))
}
