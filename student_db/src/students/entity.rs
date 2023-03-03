#[derive(Debug, Clone)]
pub struct Student {
    pub id: i32,
    pub student_id: String,
    pub name: String,
    pub gender: i32,
    pub grade: Option<i32>,
    pub note: Option<String>,
}
