use postgres::Error;

use super::entity::Student;
use crate::utils::DB_CLIENT;

pub fn select_all() -> Result<Vec<Student>, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    let res = client
        .query("select * from students", &[])?
        .iter()
        .map(|row| Student {
            id: row.get(0),
            student_id: row.get(1),
            name: row.get(2),
            gender: row.get(3),
            grade: row.get(4),
            note: row.get(5),
        })
        .collect();
    Ok(res)
}

pub fn select(id: i32) -> Result<Student, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    let row = client.query_one("select * from students where id=$1", &[&id])?;
    Ok(Student {
        id: row.get(0),
        student_id: row.get(1),
        name: row.get(2),
        gender: row.get(3),
        grade: row.get(4),
        note: row.get(5),
    })
}

pub struct CreateStudent {
    pub student_id: String,
    pub name: String,
    pub gender: i32,
    pub grade: Option<i32>,
    pub note: Option<String>,
}

pub fn create(data: CreateStudent) -> Result<Student, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    client.query_one(
        "insert into students (student_id, name, gender, grade, note) values ($1, $2, $3, $4, $5) returning *",
        &[
            &data.student_id,
            &data.name,
            &data.gender,
            &data.grade,
            &data.note,
        ],
    ).map(|row| Student {
        id: row.get(0),
        student_id: row.get(1),
        name: row.get(2),
        gender: row.get(3),
        grade: row.get(4),
        note: row.get(5),
    })
}

pub fn update(id: i32, data: CreateStudent) -> Result<Student, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    client.query_one(
        "update students set student_id=$1, name=$2, gender=$3, grade=$4, note=$5 where id=$6 returning *",
        &[
            &data.student_id,
            &data.name,
            &data.gender,
            &data.grade,
            &data.note,
            &id,
        ],
    ).map(|row| Student {
        id: row.get(0),
        student_id: row.get(1),
        name: row.get(2),
        gender: row.get(3),
        grade: row.get(4),
        note: row.get(5),
    })
}

pub fn delete(id: i32) -> Result<Student, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    client
        .query_one("delete from students where id=$1 returning *", &[&id])
        .map(|row| Student {
            id: row.get(0),
            student_id: row.get(1),
            name: row.get(2),
            gender: row.get(3),
            grade: row.get(4),
            note: row.get(5),
        })
}
