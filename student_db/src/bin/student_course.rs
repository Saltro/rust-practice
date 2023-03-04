use student_db::student_course::dao::*;

fn main() {
    let sc_list = select_all().unwrap();
    println!("All of sc:");
    sc_list.iter().for_each(|s| {
        println!("{:?}", s);
    });
    let data = CreateStudentCourse {
        student_id: 8,
        course_id: 1,
        grade: None,
    };
    match create(data) {
        Ok(s) => {
            println!("Create sc success: {}", s);
            match select(s) {
                Ok(s) => {
                    println!("Select sc: {:?}", s);
                    let data = CreateStudentCourse {
                        student_id: s.student.id,
                        course_id: 1,
                        grade: s.grade,
                    };
                    match update(s.id, data) {
                        Ok(s) => println!("Update sc success: {:?}", s),
                        Err(e) => println!("Update error: {}", e),
                    };
                    match delete(s.id) {
                        Ok(s) => println!("Delete sc success: {:?}", s),
                        Err(e) => println!("Update error: {}", e),
                    };
                },
                Err(e) => println!("{}", e),
            };
        }
        Err(e) => println!("{}", e.to_string()),
    };
}
