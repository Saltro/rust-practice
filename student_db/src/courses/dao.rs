use postgres::Error;

use super::entity::Course;
use crate::utils::DB_CLIENT;
use crate::teachers::entity::Teacher;

pub fn select_all() -> Result<Vec<Course>, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    let res = client
        .query("select courses.*, teachers.name from courses left join teachers on courses.teacher_id=teachers.id", &[])?
        .iter()
        .map(|row| Course {
            id: row.get(0),
            course_id: row.get(1),
            name: row.get(2),
            created_at: row.get(3),
            teacher: Teacher {
                id: row.get(4),
                name: row.get(5),
            },
        })
        .collect();
    Ok(res)
}

pub fn select(id: i32) -> Result<Course, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    let row = client.query_one("select courses.*, teachers.name from courses left join teachers on courses.teacher_id=teachers.id where courses.id=$1", &[&id])?;
    Ok(Course {
        id: row.get(0),
        course_id: row.get(1),
        name: row.get(2),
        created_at: row.get(3),
        teacher: Teacher {
            id: row.get(4),
            name: row.get(5),
        },
    })
}

pub struct CreateCourse {
    pub course_id: String,
    pub name: String,
    pub teacher_id: i32,
}

pub fn create(data: CreateCourse) -> Result<i32, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    client
        .query_one(
            "insert into courses (course_id, name, teacher_id) values ($1, $2, $3) returning *",
            &[&data.course_id, &data.name, &data.teacher_id],
        )
        .map(|row| row.get(0))
}

pub fn update(id: i32, data: CreateCourse) -> Result<i32, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    client
        .query_one(
            "update courses set course_id=$1, name=$2, teacher_id=$3 where id=$4 returning *",
            &[&data.course_id, &data.name, &data.teacher_id, &id],
        )
        .map(|row| row.get(0))
}

pub fn delete(id: i32) -> Result<i32, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    client
        .query_one("delete from courses where id=$1 returning *", &[&id])
        .map(|row| row.get(0))
}
