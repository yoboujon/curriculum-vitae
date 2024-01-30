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
    pub profile_pic: Option<String>
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
    pub picture_url: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct Project {
    pub id: Option<i32>,
    pub date_done: Option<NaiveDate>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub github_link: Option<String>,
    pub picture_name: Option<String>,
    pub type_project: Option<String>,
    pub report_link: Option<String>,
    pub archive_link: Option<String>,
    pub app_link: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct ProgrammingLanguages {
    pub lang: String,
    pub icon: String,
    pub type_icon: String,
    pub color: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct Softwares {
    pub software: String,
    pub icon: String,
    pub type_icon: String,
    pub color: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct Languages {
    pub lang: String,
    pub icon_alpha: String,
    pub level: String
}

#[derive(Deserialize, Serialize)]
pub struct Tags {
    pub lang: Option<String>,
    pub icon: Option<String>,
    pub type_icon: Option<String>,
    pub color: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct AllTags {
    pub project_id: Option<i32>,
    pub lang: Option<String>,
    pub icon: Option<String>,
    pub type_icon: Option<String>,
    pub color: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct SimpleProject {
    pub project_id: Option<i32>,
    pub title: Option<String>
}