use postgres::{Client, Error, NoTls};

use super::entity::Course;

pub fn select_all() -> Result<Vec<Course>, Error> {
    let mut client = Client::connect(
        "host=localhost user=postgres password=111111 dbname=student_db",
        NoTls,
    )?;
    let res = client
        .query("select * from courses", &[])?
        .iter()
        .map(|row| Course {
            id: row.get(0),
            course_id: row.get(1),
            name: row.get(2),
            created_at: row.get(3),
            teacher_id: row.get(4),
        })
        .collect();
    Ok(res)
}

pub fn select(id: i32) -> Result<Course, Error> {
    let mut client = Client::connect(
        "host=localhost user=postgres password=111111 dbname=student_db",
        NoTls,
    )?;
    let row = client.query_one("select * from courses where id=$1", &[&id])?;
    Ok(Course {
        id: row.get(0),
        course_id: row.get(1),
        name: row.get(2),
        created_at: row.get(3),
        teacher_id: row.get(4),
    })
}

pub struct CreateCourse {
    pub course_id: String,
    pub name: String,
    pub teacher_id: i32,
}

pub fn create(data: CreateCourse) -> Result<Course, Error> {
    let mut client = Client::connect(
        "host=localhost user=postgres password=111111 dbname=student_db",
        NoTls,
    )?;
    client.query_one(
        "insert into courses (course_id, name, teacher_id) values ($1, $2, $3) returning *",
        &[
            &data.course_id,
            &data.name,
            &data.teacher_id,
        ],
    ).map(|row| Course {
        id: row.get(0),
        course_id: row.get(1),
        name: row.get(2),
        created_at: row.get(3),
        teacher_id: row.get(4),
    })
}

pub fn update(id: i32, data: CreateCourse) -> Result<Course, Error> {
    let mut client = Client::connect(
        "host=localhost user=postgres password=111111 dbname=student_db",
        NoTls,
    )?;
    client.query_one(
        "update courses set course_id=$1, name=$2, teacher_id=$3 where id=$4 returning *",
        &[
            &data.course_id,
            &data.name,
            &data.teacher_id,
            &id,
        ],
    ).map(|row| Course {
        id: row.get(0),
        course_id: row.get(1),
        name: row.get(2),
        created_at: row.get(3),
        teacher_id: row.get(4),
    })
}

pub fn delete(id: i32) -> Result<Course, Error> {
    let mut client = Client::connect(
        "host=localhost user=postgres password=111111 dbname=student_db",
        NoTls,
    )?;
    client
        .query_one("delete from courses where id=$1 returning *", &[&id])
        .map(|row| Course {
            id: row.get(0),
            course_id: row.get(1),
            name: row.get(2),
            created_at: row.get(3),
            teacher_id: row.get(4),
        })
}
