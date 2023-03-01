use postgres::{Client, Error, NoTls};

use super::entity::Student;

pub fn select_all() -> Result<Vec<Student>, Error> {
    let mut client = Client::connect("host=localhost user=postgres password=111111 dbname=student_db", NoTls)?;
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
    let mut client = Client::connect("host=localhost user=postgres password=111111 dbname=student_db", NoTls)?;
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

pub fn create(data: CreateStudent) -> Result<u64, Error> {
    let mut client = Client::connect("host=localhost user=postgres password=111111 dbname=student_db", NoTls)?;
    client.execute(
        "insert into students (student_id, name, gender, grade, note) values ($1, $2, $3, $4, $5)",
        &[
            &data.student_id,
            &data.name,
            &data.gender,
            &data.grade,
            &data.note,
        ],
    )
}

pub struct UpdateStudent {
    pub student_id: Option<String>,
    pub name: Option<String>,
    pub gender: Option<i32>,
    pub grade: Option<i32>,
    pub note: Option<String>,
}

pub fn update(id: i32, data: UpdateStudent) -> Result<u64, Error> {
    let mut client = Client::connect("host=localhost user=postgres password=111111 dbname=student_db", NoTls)?;
    client.execute(
        "update students set student_id=$1, name=$2, gender=$3, grade=$4, note=$5 where id=$6",
        &[
            &data.student_id,
            &data.name,
            &data.gender,
            &data.grade,
            &data.note,
            &id,
        ],
    )
}
