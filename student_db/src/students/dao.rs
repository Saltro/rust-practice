use mysql::{
    prelude::{BinQuery, TextQuery, WithParams},
    Pool,
};

use super::entity::Student;

pub fn select_all() -> Result<Vec<Student>, String> {
    let url = "mysql://rust:123456@localhost:3306/student-db";
    let pool = Pool::new(url).unwrap();
    let conn = pool.get_conn().unwrap();
    let res = "select * from students".map(conn, |(id, student_id, name, gender, grade, note)| {
        Student {
            id,
            student_id,
            name,
            gender,
            grade,
            note,
        }
    });
    match res {
        Ok(v) => Ok(v),
        Err(_) => Err("Mysql error".to_string()),
    }
}

pub fn select(id: u64) -> Result<Student, String> {
    let url = "mysql://rust:123456@localhost:3306/student-db";
    let pool = Pool::new(url).unwrap();
    let conn = pool.get_conn().unwrap();
    let res = "select * from students where id=?".with((id,)).first(conn);
    match res {
        Ok(s) => match s {
            Some((id, student_id, name, gender, grade, note)) => Ok(Student {
                id,
                student_id,
                name,
                gender,
                grade,
                note,
            }),
            None => Err("Student not found".to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

pub struct CreateStudent {
    pub student_id: String,
    pub name: String,
    pub gender: i32,
    pub grade: Option<i32>,
    pub note: Option<String>,
}

pub fn create(data: CreateStudent) -> Result<u64, String> {
    let url = "mysql://rust:123456@localhost:3306/student-db";
    let pool = Pool::new(url).unwrap();
    let conn = pool.get_conn().unwrap();
    let res = "insert into students (student_id, name, gender, grade, note) values (?, ?, ?, ?, ?)"
        .with((
            data.student_id,
            data.name,
            data.gender,
            data.grade,
            data.note,
        ))
        .run(conn);
    match res {
        Ok(s) => match s.last_insert_id() {
            Some(i) => Ok(i),
            None => Err("Create error".to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

pub struct UpdateStudent {
    pub student_id: Option<String>,
    pub name: Option<String>,
    pub gender: Option<i32>,
    pub grade: Option<i32>,
    pub note: Option<String>,
}

pub fn update(id: u64, data: UpdateStudent) -> Result<u64, String> {
    let url = "mysql://rust:123456@localhost:3306/student-db";
    let pool = Pool::new(url).unwrap();
    let conn = pool.get_conn().unwrap();
    let res = "update students set student_id=?, name=?, gender=?, grade=?, note=? where id=?"
        .with((
            data.student_id,
            data.name,
            data.gender,
            data.grade,
            data.note,
            id,
        ))
        .run(conn);
    match res {
        Ok(s) => Ok(s.affected_rows()),
        Err(e) => Err(e.to_string()),
    }
}
