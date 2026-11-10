use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    pub id: i64,
    pub entity_type: String,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub pet_name: Option<String>,
    pub gender: Option<String>,
    pub photo_path: Option<String>,

    pub birth_year: Option<i32>,
    pub birth_year_qual: Option<String>,
    pub birth_month: Option<i32>,
    pub birth_month_qual: Option<String>,
    pub birth_day: Option<i32>,
    pub birth_day_qual: Option<String>,

    pub death_year: Option<i32>,
    pub death_year_qual: Option<String>,
    pub death_month: Option<i32>,
    pub death_month_qual: Option<String>,
    pub death_day: Option<i32>,
    pub death_day_qual: Option<String>,
}