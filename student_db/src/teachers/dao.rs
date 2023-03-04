use postgres::Error;

use super::entity::Teacher;
use crate::utils::DB_CLIENT;

pub fn select_all() -> Result<Vec<Teacher>, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    let res = client
        .query("select * from teachers", &[])?
        .iter()
        .map(|row| Teacher {
            id: row.get(0),
            name: row.get(1),
        })
        .collect();
    Ok(res)
}

pub fn select(id: i32) -> Result<Teacher, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    let row = client.query_one("select * from teachers where id=$1", &[&id])?;
    Ok(Teacher {
        id: row.get(0),
        name: row.get(1),
    })
}

pub struct CreateTeacher {
    pub name: String,
}

pub fn create(data: CreateTeacher) -> Result<Teacher, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    client
        .query_one(
            "insert into teachers (name) values ($1) returning *",
            &[&data.name],
        )
        .map(|row| Teacher {
            id: row.get(0),
            name: row.get(1),
        })
}

pub fn update(id: i32, data: CreateTeacher) -> Result<Teacher, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    client
        .query_one(
            "update teachers set name=$1 where id=$2 returning *",
            &[&data.name, &id],
        )
        .map(|row| Teacher {
            id: row.get(0),
            name: row.get(1),
        })
}

pub fn delete(id: i32) -> Result<Teacher, Error> {
    let mut client = DB_CLIENT.lock().unwrap();
    client
        .query_one("delete from teachers where id=$1 returning *", &[&id])
        .map(|row| Teacher {
            id: row.get(0),
            name: row.get(1),
        })
}
