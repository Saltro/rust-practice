use student_db::teachers::dao::*;

fn main() {
    let teacher_list = select_all().unwrap();
    println!("All of teachers:");
    teacher_list.iter().for_each(|s| {
        println!("{:?}", s);
    });
    let data = CreateTeacher {
        name: "lixiang".to_string(),
    };
    match create(data) {
        Ok(s) => {
            println!("Create teacher success: {:?}", s);
            let data = CreateTeacher {
                name: "lili".to_string(),
            };
            match update(s.id, data) {
                Ok(s) => println!("Update teacher success: {:?}", s),
                Err(e) => println!("Update error: {}", e),
            };
            match delete(s.id) {
                Ok(s) => println!("Delete teacher success: {:?}", s),
                Err(e) => println!("Update error: {}", e),
            };
        }
        Err(e) => println!("{}", e.to_string()),
    };
}
