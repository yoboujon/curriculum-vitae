use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDate;

#[derive(Deserialize, Serialize)]
pub struct Info {
    pub id: i64,
    pub full_name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub softskills: Option<String>,
    pub interests: Option<String>,
    pub birth_year: Option<NaiveDate>,
}

#[derive(Deserialize, Serialize)]
pub struct Education {
    pub id: i64,
    pub start_year: Option<NaiveDate>,
    pub end_year: Option<NaiveDate>,
    pub school: Option<String>,
    pub speciality: Option<String>,
    pub school_location: Option<String>,
    pub school_options: Option<String>,
    pub picture_url: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct Experience {
    pub id: i64,
    pub job_position: Option<String>,
    pub job_description: Option<String>,
    pub enterprise: Option<String>,
    pub enterprise_location: Option<String>,
    pub start_year: Option<NaiveDate>,
    pub end_year: Option<NaiveDate>,
}

#[derive(Deserialize, Serialize)]
pub struct Project {
    pub id: i64,
    pub date_done: Option<NaiveDate>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub github_link: Option<String>,
    pub id_skills: i64,
}

#[derive(Deserialize, Serialize)]
pub struct Skills {
    pub id: i64,
    pub programming_lang: Option<String>,
    pub software: Option<String>,
    pub languages: Option<String>,
}