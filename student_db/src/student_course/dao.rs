use postgres::Error;

use super::entity::StudentCourse;
use crate::courses::entity::Course;
use crate::students::entity::Student;
use crate::teachers::entity::Teacher;
use crate::utils::DB_CLIENT;

pub fn select_all() -> Result<Vec<StudentCourse>, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    let res = client
        .query("
            select SC.*, S.student_id as s_id, S.name as s_name, S.gender, S.grade, S.note, C.course_id as c_id, C.name as c_name, C.created_at, C.teacher_id, T.name
            from student_course as SC
            left join students as S on SC.student_id=S.id
            left join courses as C on SC.course_id=C.id
            left join teachers as T on C.teacher_id=T.id
        ", &[])?
        .iter()
        .map(|row| StudentCourse {
            id: row.get(0),
            student: Student {
                id: row.get(1),
                student_id: row.get(4),
                name: row.get(5),
                gender: row.get(6),
                grade: row.get(7),
                note: row.get(8),
            },
            course: Course {
                id: row.get(2),
                course_id: row.get(9),
                name: row.get(10),
                created_at: row.get(11),
                teacher: Teacher {
                    id: row.get(12),
                    name: row.get(13),
                },
            },
            grade: row.get(3),
        })
        .collect();
    Ok(res)
}

pub fn select(id: i32) -> Result<StudentCourse, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    let row = client.query_one(
        "
            select SC.*, S.student_id as s_id, S.name as s_name, S.gender, S.grade, S.note, C.course_id as c_id, C.name as c_name, C.created_at, C.teacher_id, T.name
            from student_course as SC
            left join students as S on SC.student_id=S.id
            left join courses as C on SC.course_id=C.id
            left join teachers as T on C.teacher_id=T.id
            where SC.id=$1
        ",
        &[&id])?;
    Ok(StudentCourse {
        id: row.get(0),
        student: Student {
            id: row.get(1),
            student_id: row.get(4),
            name: row.get(5),
            gender: row.get(6),
            grade: row.get(7),
            note: row.get(8),
        },
        course: Course {
            id: row.get(2),
            course_id: row.get(9),
            name: row.get(10),
            created_at: row.get(11),
            teacher: Teacher {
                id: row.get(12),
                name: row.get(13),
            },
        },
        grade: row.get(3),
    })
}

pub struct CreateStudentCourse {
    pub student_id: i32,
    pub course_id: i32,
    pub grade: Option<f32>,
}

pub fn create(data: CreateStudentCourse) -> Result<i32, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    client.query_one(
        "insert into student_course (student_id, course_id, grade) values ($1, $2, $3) returning *",
        &[
            &data.student_id,
            &data.course_id,
            &data.grade,
        ],
    ).map(|row| row.get(0))
}

pub fn update(id: i32, data: CreateStudentCourse) -> Result<i32, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    client.query_one(
        "update student_course set student_id=$1, course_id=$2, grade=$3 where id=$4 returning *",
        &[
            &data.student_id,
            &data.course_id,
            &data.grade,
            &id,
        ],
    ).map(|row| row.get(0))
}

pub fn delete(id: i32) -> Result<i32, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    client
        .query_one("delete from student_course where id=$1 returning *", &[&id])
        .map(|row| row.get(0))
}
