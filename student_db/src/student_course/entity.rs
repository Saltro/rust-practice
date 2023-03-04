use crate::courses::entity::Course;
use crate::students::entity::Student;

#[derive(Debug, Clone)]
pub struct StudentCourse {
    pub id: i32,
    pub student: Student,
    pub course: Course,
    pub grade: Option<f32>,
}
