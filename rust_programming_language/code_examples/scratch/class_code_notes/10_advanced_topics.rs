
// Circular References
/*
Assume our example is many students to many courses
*/

// struct OGStudent<'a> {
//     name: String,
//     courses: Vec<&'a OGCourse<'a>>
// }

// impl<'a> OGStudent<'a> {
//     fn new(name: &str) -> OGStudent<'a> {
//         OGStudent {
//             name: name.into(),
//             courses: Vec::new()
//         }
//     }
// }

// struct OGCourse<'a> {
//     name: String, 
//     students: Vec<&'a OGStudent<'a>>
// }

// impl<'a> OGCourse<'a> {
//     fn new(name: &str) -> OGCourse<'a> {
//         OGCourse {
//             name: name.into(),
//             students: Vec::new()
//         }
//     }

//     fn add_student(&'a mut self, student: &'a mut OGStudent<'a>) {
//         student.courses.push(self);
//         self.students.push(student);
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

struct Student {
    name: String,
    courses: Vec<Rc<RefCell<Course>>>
}

impl Student {
    fn new(name: &str) -> Student {
        Student {
            name: name.into(),
            courses: Vec::new()
        }
    }
}

struct Course {
    name: String, 
    students: Vec<Rc<RefCell<Student>>>
}

impl Course {
    fn new(name: &str) -> Course {
        Course {
            name: name.into(),
            students: Vec::new()
        }
    }

    fn add_student(course: Rc<RefCell<Course>>, student: Rc<RefCell<Student>>) {
        student.borrow_mut().courses.push(course.clone());
        course.borrow_mut().students.push(student);
    }
}

fn main () {
    // let john = OGStudent::new("John");
    // let course = OGCourse::new("Rust Course");
    // course.add_student(john);

    // nope that's going to be horrendous so let's use a lifetime. 
    let john = Rc::new(
        RefCell::new(
            Student::new("John")
        )
    );
    let course = Course::new("Rust Course");
    let magic_course = Rc::new(RefCell::new(course));
    // course.add_student(john); // Rc
    Course::add_student(magic_course, john);
    
}