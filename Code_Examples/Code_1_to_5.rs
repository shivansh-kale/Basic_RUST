use std::collections::HashMap;

//======================================================
// MODULE 1 : BASICS
//======================================================

// Constant
const COLLEGE_NAME: &str = "University";

// Function
fn greet(name: &str) {
    println!("Welcome, {}!", name);
}

// Function returning Result (Module 4)
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

//======================================================
// MODULE 4 : ENUMS
//======================================================

#[derive(Debug, Clone)]
enum Department {
    AI,
    IT,
    Computer,
    Mechanical,
}

//======================================================
// MODULE 3 : STRUCTS
//======================================================

#[derive(Debug, Clone)]
struct Student {
    id: u32,
    name: String,
    age: u8,
    department: Department,
    marks: Vec<u32>,
}

impl Student {
    //--------------------------------------------------
    // Associated Function (Constructor)
    //--------------------------------------------------
    fn new(id: u32, name: String, age: u8, department: Department) -> Self {
        Self {
            id,
            name,
            age,
            department,
            marks: Vec::new(),
        }
    }

    //--------------------------------------------------
    // Method using mutable reference
    //--------------------------------------------------
    fn add_mark(&mut self, mark: u32) {
        self.marks.push(mark);
    }

    //--------------------------------------------------
    // Method using immutable reference
    //--------------------------------------------------
    fn average(&self) -> Option<f32> {
        if self.marks.is_empty() {
            return None;
        }

        let total: u32 = self.marks.iter().sum();

        Some(total as f32 / self.marks.len() as f32)
    }

    //--------------------------------------------------
    // Print Details
    //--------------------------------------------------
    fn display(&self) {
        println!("----------------------------");
        println!("ID         : {}", self.id);
        println!("Name       : {}", self.name);
        println!("Age        : {}", self.age);
        println!("Department : {:?}", self.department);
        println!("Marks      : {:?}", self.marks);

        match self.average() {
            Some(avg) => println!("Average    : {:.2}", avg),
            None => println!("Average    : No Marks"),
        }

        println!("----------------------------");
    }
}

//======================================================
// MODULE 2 : OWNERSHIP
//======================================================

// Ownership moves into this function
fn take_ownership(student: Student) {
    println!("Ownership moved to function.");
    println!("{}", student.name);
}

// Borrowing (Immutable)
fn print_student(student: &Student) {
    println!("Borrowed Student: {}", student.name);
}

// Borrowing (Mutable)
fn add_bonus_marks(student: &mut Student) {
    student.add_mark(100);
}

//======================================================
// MAIN
//======================================================

fn main() {
    println!("===============================");
    println!("{}", COLLEGE_NAME);
    println!("===============================");

    //--------------------------------------------------
    // Variables
    //--------------------------------------------------

    let college = "Engineering";
    let mut year = 2;

    println!("College : {}", college);
    println!("Year    : {}", year);

    year += 1;

    println!("Updated Year : {}", year);

    //--------------------------------------------------
    // Strings
    //--------------------------------------------------

    let student_name = String::from("Shivansh");

    greet(&student_name);

    //--------------------------------------------------
    // Create Struct
    //--------------------------------------------------

    let mut student =
        Student::new(101, student_name, 20, Department::AI);

    student.add_mark(95);
    student.add_mark(90);
    student.add_mark(85);

    student.display();

    //--------------------------------------------------
    // Ownership & Borrowing
    //--------------------------------------------------

    print_student(&student);

    add_bonus_marks(&mut student);

    println!("After Bonus:");

    student.display();

    //--------------------------------------------------
    // Clone
    //--------------------------------------------------

    let cloned_student = student.clone();

    println!("Clone Created");

    cloned_student.display();

    //--------------------------------------------------
    // Ownership Move
    //--------------------------------------------------

    take_ownership(cloned_student);

    // cloned_student cannot be used here.

    //--------------------------------------------------
    // Option
    //--------------------------------------------------

    match student.average() {
        Some(avg) => println!("Average = {:.2}", avg),
        None => println!("No marks available"),
    }

    //--------------------------------------------------
    // Result
    //--------------------------------------------------

    match divide(20.0, 5.0) {
        Ok(answer) => println!("Division = {}", answer),
        Err(error) => println!("Error = {}", error),
    }

    //--------------------------------------------------
    // Vec
    //--------------------------------------------------

    let numbers = vec![10, 20, 30, 40, 50];

    println!("\nVector Elements:");

    for value in &numbers {
        println!("{}", value);
    }

    //--------------------------------------------------
    // HashMap
    //--------------------------------------------------

    let mut student_database: HashMap<u32, Student> = HashMap::new();

    student_database.insert(student.id, student);

    println!("\nStudents in Database:");

    for (id, stu) in &student_database {
        println!("{} -> {}", id, stu.name);
    }

    //--------------------------------------------------
    // HashMap Search
    //--------------------------------------------------

    let search_id = 101;

    match student_database.get(&search_id) {
        Some(student) => {
            println!("\nStudent Found");
            student.display();
        }
        None => println!("Student Not Found"),
    }

    //--------------------------------------------------
    // Iterate Over Marks
    //--------------------------------------------------

    if let Some(student) = student_database.get(&101) {
        println!("\nMarks:");

        for mark in &student.marks {
            println!("{}", mark);
        }
    }

    println!("\nProgram Finished Successfully.");
}