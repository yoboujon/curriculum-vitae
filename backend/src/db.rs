use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDate;

#[derive(Deserialize, Serialize, Copy, Clone)]
pub struct LangId {
    pub id: Option<i32>
}

#[derive(Deserialize, Serialize)]
pub struct Info {
    pub full_name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub birth_year: Option<NaiveDate>,
    pub profile_pic: Option<String>,
    pub title: Option<String>,
    pub softskills: Option<String>,
    pub interests: Option<String>,
    pub description: Option<String>,
    pub pdf_url: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct Education {
    pub school: Option<String>,
    pub start_year: Option<NaiveDate>,
    pub end_year: Option<NaiveDate>,
    pub picture_url: Option<String>,
    pub speciality: Option<String>,
    pub school_options: Option<String>,
    pub school_location: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct Experience {
    pub enterprise: Option<String>,
    pub start_year: Option<NaiveDate>,
    pub end_year: Option<NaiveDate>,
    pub picture_url: Option<String>,
    pub job_position: Option<String>,
    pub job_description: Option<String>,
    pub enterprise_location: Option<String>
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
    pub app_link: Option<String>,
    pub short_description: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct Categories {
    pub id: Option<i32>,
    pub name: Option<String>
}

#[derive(Deserialize,Serialize,Clone)]
pub struct CategoriesText {
    pub icon: Option<String>,
    pub type_icon: Option<String>,
    pub name: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct Skills {
    pub id: Option<i32>,
    pub skill: Option<String>,
    pub icon: Option<String>,
    pub type_icon: Option<String>,
    pub color: Option<String>,
    pub is_shown: Option<bool>
}

#[derive(Deserialize, Serialize)]
pub struct Languages {
    pub lang: Option<String>,
    pub icon_alpha: Option<String>,
    pub level: Option<String>,
    pub url_name: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct AllSkills {
    pub project_id: Option<i32>,
    pub skill: Option<String>,
    pub icon: Option<String>,
    pub type_icon: Option<String>,
    pub color: Option<String>,
    pub is_shown: Option<bool>
}

#[derive(Deserialize, Serialize)]
pub struct SimpleProject {
    pub project_id: Option<i32>,
    pub title: Option<String>
}

#[derive(Deserialize,Serialize)]
pub struct Id {
    pub id: Option<i32>
}