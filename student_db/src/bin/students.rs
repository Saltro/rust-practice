use student_db::students::dao::*;

fn main() {
    let student_list = select_all().unwrap();
    println!("All of students:");
    student_list.iter().for_each(|s| {
        println!("{:?}", s);
    });
    let data = CreateStudent {
        student_id: "10004".to_string(),
        name: "lixiang".to_string(),
        gender: 1,
        grade: None,
        note: Some("Test user 2".to_string()),
    };
    match create(data) {
        Ok(s) => {
            println!("Create student success: {:?}", s);
            let data = UpdateStudent {
                student_id: Some(s.student_id),
                name: Some(s.name),
                gender: Some(s.gender),
                grade: s.grade,
                note: Some("Test user".to_string()),
            };
            match update(s.id, data) {
                Ok(s) => println!("Update student success: {:?}", s),
                Err(e) => println!("Update error: {}", e),
            };
            match delete(s.id) {
                Ok(s) => println!("Delete student success: {:?}", s),
                Err(e) => println!("Update error: {}", e),
            };
        }
        Err(e) => println!("{}", e.to_string()),
    };
}
