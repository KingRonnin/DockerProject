use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use crate::schema::students;

#[derive(Queryable, Serialize)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub course: String,
    pub present_date: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "students"]
pub struct NewStudent {
    pub name: String,
    pub course: String,
    pub present_date: Option<NaiveDateTime>,
}