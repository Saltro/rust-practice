use student_db::courses::dao::*;

fn main() {
    let course_list = select_all().unwrap();
    println!("All of courses:");
    course_list.iter().for_each(|s| {
        println!("{:?}", s);
    });
    let data = CreateCourse {
        course_id: "10001".to_string(),
        name: "编译原理".to_string(),
        teacher_id: 1,
    };
    match create(data) {
        Ok(s) => {
            println!("Create course success: {}", s);
            let course = select(s).unwrap();
            println!("Select course: {:?}", s);
            let data = CreateCourse {
                course_id: course.course_id,
                name: course.name,
                teacher_id: 2,
            };
            match update(s, data) {
                Ok(s) => println!("Update course success: {}", s),
                Err(e) => println!("Update error: {}", e),
            };
            match delete(s) {
                Ok(s) => println!("Delete course success: {}", s),
                Err(e) => println!("Update error: {}", e),
            };
        }
        Err(e) => println!("{}", e.to_string()),
    };
}
