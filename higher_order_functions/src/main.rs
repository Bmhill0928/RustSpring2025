//Create a struct Student(major)
struct Student {
    major: String,
}

//Higher order functions update majors
fn update_majors(mut collection: Vec<Student>, behavior: fn(&mut Student, String)) {

    for student in collection.iter_mut(){
        behavior(student, String::from("Computer Science"));
    }

    for(i, student) in collection.iter().enumerate(){
        println!("Student {}: Major - {}", i + 1, student.major);
    }
}

//First Order functions, assign_major
fn assign_major(s: &mut Student, major: String) {
    s.major = major;
}



fn main(){

    //Create vector of students
    let students = vec![
        Student { major: String::from("Undeclared Major")},
        Student { major: String::from("Undeclared Major")},
        Student { major: String::from("Undeclared Major")},
    ];

    //Update the students majors
    update_majors(students, assign_major);

}