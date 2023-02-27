use mysql::{Pool, prelude::Queryable};

use super::entity::Student;

pub fn select_all() -> Vec<Student> {
    let url = "mysql://rust:123456@localhost:3306/student-db";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let res = conn
        .query_map(
            "select * from students",
            |(id, student_id, name, gender, grade, note)| Student {
                id,
                student_id,
                name,
                gender,
                grade,
                note,
            },
        )
        .expect("Error");
    res
}

pub fn select(id: i32) -> Result<Student, &'static str> {
    let url = "mysql://rust:123456@localhost:3306/student-db";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let res = conn.query_first(format!("select * from students where id={}", id))
        .map(|row| {
            row.map(|(id, student_id, name, gender, grade, note)| Student {
                id,
                student_id,
                name,
                gender,
                grade,
                note,
            })
        }).expect("Error");
    return match res {
        Some(s) => Ok(s),
        None => Err("Not found")
    }
}
