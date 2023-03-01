use student_db::students::dao::*;

fn main() {
    let student_list = select_all().unwrap();
    student_list.iter().for_each(|s| {
        println!("{:?}", s);
    });
    let sid = 1;
    let s = select(sid);
    match s {
        Ok(s) => {
            let data = UpdateStudent {
                student_id: Some(s.student_id),
                name: Some("lixiang".to_string()),
                gender: Some(s.gender),
                grade: s.grade,
                note: Some("Test user".to_string()),
            };
            let res = update(sid, data);
            match res {
                Ok(s) => println!("Update success: {}", s),
                Err(e) => println!("Update error: {}", e),
            };
        },
        Err(_) => {},
    };
}
