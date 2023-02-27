mod students;

fn main() {
    let student_list = students::dao::select_all();
    student_list.iter().for_each(|s| {
        println!("{}, {}, {}, {}, {}, {:?}", s.id, s.student_id, s.name, s.gender, s.grade, s.note);
    });
    let student = students::dao::select(1).unwrap();
    println!("{}, {}, {}, {}, {}, {:?}", student.id, student.student_id, student.name, student.gender, student.grade, student.note);
}
