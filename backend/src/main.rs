use axum::{
    extract::{Path, State},
    routing::{get, post},
    Form, Json, Router,
};
use axum_error::Result;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    types::chrono::NaiveDate,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

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
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    // Server
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    Ok(axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await?)
}

#[derive(Deserialize, Serialize)]
struct Info {
    id: i64,
    full_name: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    softskills: Option<String>,
    interests: Option<String>,
    birth_year: Option<NaiveDate>,
}

async fn info(State(pool): State<PgPool>) -> Result<Json<Vec<Info>>> {
    let infos = sqlx::query_as!(
        Info,
        "SELECT id, full_name, phone_number, email, softskills, interests, birth_year FROM public.info"
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(infos))
}
