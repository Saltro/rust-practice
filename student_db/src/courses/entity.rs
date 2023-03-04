use chrono::NaiveDateTime;

use crate::teachers::entity::Teacher;

#[derive(Debug, Clone)]
pub struct Course {
    pub id: i32,
    pub course_id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub teacher: Teacher,
}
