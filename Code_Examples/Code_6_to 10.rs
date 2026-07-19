// =============================================================
// main.rs (Part 1)
// Modules Covered:
// 6 - Traits
// 7 - Generics
// 8 - Error Handling (Beginning)
// =============================================================

use std::fmt::Debug;

// =========================
// Module 6 : Traits
// =========================

trait Printable {
    fn print(&self);

    fn summary(&self) {
        println!("Default Summary from Printable trait.");
    }
}

#[derive(Debug, Clone)]
struct Student {
    id: u32,
    name: String,
    cgpa: f32,
}

impl Printable for Student {
    fn print(&self) {
        println!("---------------------");
        println!("ID   : {}", self.id);
        println!("Name : {}", self.name);
        println!("CGPA : {:.2}", self.cgpa);
        println!("---------------------");
    }
}

#[derive(Debug)]
struct Course {
    code: String,
    credits: u8,
}

impl Printable for Course {
    fn print(&self) {
        println!("Course : {}", self.code);
        println!("Credits: {}", self.credits);
    }
}

fn print_anything<T: Printable>(item: &T) {
    item.print();
    item.summary();
}

// =========================
// Module 7 : Generics
// =========================

#[derive(Debug)]
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Self { value }
    }

    fn get(&self) -> &T {
        &self.value
    }
}

fn largest<T>(a: T, b: T) -> T
where
    T: PartialOrd + Copy,
{
    if a > b {
        a
    } else {
        b
    }
}

fn display_debug<T: Debug>(value: &T) {
    println!("{:?}", value);
}

#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T: Debug, U: Debug> Pair<T, U> {
    fn show(&self) {
        println!("{:?} {:?}", self.first, self.second);
    }
}

// =========================
// Module 8 : Error Handling
// =========================

#[derive(Debug)]
enum AppError {
    DivisionByZero,
    NegativeNumber,
}

fn divide(a: f64, b: f64) -> Result<f64, AppError> {
    if b == 0.0 {
        Err(AppError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn square_root(num: f64) -> Result<f64, AppError> {
    if num < 0.0 {
        Err(AppError::NegativeNumber)
    } else {
        Ok(num.sqrt())
    }
}

fn perform_calculation(a: f64, b: f64) -> Result<f64, AppError> {
    let result = divide(a, b)?;
    let answer = square_root(result)?;
    Ok(answer)
}

fn find_student<'a>(
    students: &'a Vec<Student>,
    id: u32,
) -> Option<&'a Student> {
    students.iter().find(|s| s.id == id)
}

fn main() {
    println!("=== Traits ===");

    let student = Student {
        id: 101,
        name: String::from("Shivansh"),
        cgpa: 9.45,
    };

    let course = Course {
        code: String::from("CS401"),
        credits: 4,
    };

    print_anything(&student);
    print_anything(&course);

    println!("\n=== Generics ===");

    let int_box = Container::new(100);
    let text_box = Container::new(String::from("Rust"));

    println!("Integer: {}", int_box.get());
    println!("String : {}", text_box.get());

    println!("Largest: {}", largest(20, 50));

    let pair = Pair {
        first: "Rust",
        second: 2026,
    };

    pair.show();

    display_debug(&student);

    println!("\n=== Result ===");

    match perform_calculation(100.0, 4.0) {
        Ok(value) => println!("Answer = {:.2}", value),
        Err(error) => println!("Error = {:?}", error),
    }

    match perform_calculation(10.0, 0.0) {
        Ok(value) => println!("{value}"),
        Err(error) => println!("Handled Error: {:?}", error),
    }

    println!("\n=== Option ===");

    let students = vec![
        student.clone(),
        Student {
            id: 102,
            name: String::from("Alice"),
            cgpa: 8.9,
        },
        Student {
            id: 103,
            name: String::from("Bob"),
            cgpa: 8.1,
        },
    ];

    match find_student(&students, 102) {
        Some(s) => {
            println!("Student Found");
            s.print();
        }
        None => println!("Student Not Found"),
    }

    println!("\nEnd of Part 1");
    println!("Paste Part 2 directly below this file.");
}



// =============================================================
// main.rs (Part 2)
// Continue past Part 1 by pasting this directly below it.
// Modules Covered:
// 8 - Error Handling (Advanced)
// 9 - Lifetimes
// 10 - Module-like Organization
// =============================================================

// -------------------------------------------------------------
// More Error Handling
// -------------------------------------------------------------

fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err(String::from("Cannot divide by zero."));
    }

    Ok(a / b)
}

fn print_result(result: Result<f64, String>) {
    match result {
        Ok(value) => println!("Success : {:.2}", value),
        Err(err) => println!("Failure : {}", err),
    }
}

fn demonstrate_result_methods() {
    println!("\n=== Result Helper Methods ===");

    let value = safe_divide(100.0, 5.0)
        .map(|x| x * 2.0)
        .unwrap_or(-1.0);

    println!("Mapped Value : {}", value);

    let failed = safe_divide(100.0, 0.0)
        .unwrap_or_else(|_| 0.0);

    println!("Fallback Value : {}", failed);
}

// -------------------------------------------------------------
// Option<T>
// -------------------------------------------------------------

fn highest_mark(marks: &[u32]) -> Option<u32> {
    marks.iter().copied().max()
}

fn demonstrate_option() {
    println!("\n=== Option Demo ===");

    let marks = vec![78, 91, 85, 99, 88];

    match highest_mark(&marks) {
        Some(mark) => println!("Highest Mark : {}", mark),
        None => println!("No Marks Found"),
    }
}

// -------------------------------------------------------------
// Module 9 : Lifetimes
// -------------------------------------------------------------

#[derive(Debug)]
struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

impl<'a> Book<'a> {
    fn display(&self) {
        println!("Book   : {}", self.title);
        println!("Author : {}", self.author);
    }
}

fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

fn lifetime_demo() {
    println!("\n=== Lifetimes ===");

    let rust = String::from("Rust Programming");
    let solana = String::from("Solana");

    let answer = longest(&rust, &solana);

    println!("Longest String : {}", answer);

    let book = Book {
        title: "Rust Book",
        author: "Steve",
    };

    book.display();
}

static LANGUAGE: &str = "Rust";

fn static_demo() {
    println!("\n=== 'static Lifetime ===");
    println!("{}", LANGUAGE);
}

// -------------------------------------------------------------
// Module 10 : Modules (Simulated)
// -------------------------------------------------------------

mod math_utils {

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn square(x: i32) -> i32 {
        x * x
    }

    pub fn cube(x: i32) -> i32 {
        x * x * x
    }
}

mod text_utils {

    pub fn capitalize(name: &str) -> String {
        let mut chars = name.chars();

        match chars.next() {
            Some(first) => {
                first.to_uppercase().collect::<String>() + chars.as_str()
            }
            None => String::new(),
        }
    }

    pub fn repeat(text: &str, times: usize) -> String {
        text.repeat(times)
    }
}

fn module_demo() {
    println!("\n=== Module Demo ===");

    println!("10 + 20 = {}", math_utils::add(10, 20));
    println!("Square = {}", math_utils::square(12));
    println!("Cube = {}", math_utils::cube(4));

    println!("{}", text_utils::capitalize("shivansh"));
    println!("{}", text_utils::repeat("Rust ", 3));
}

// -------------------------------------------------------------
// Generic Function with Trait Bound
// -------------------------------------------------------------

fn show_item<T>(item: T)
where
    T: std::fmt::Debug,
{
    println!("{:?}", item);
}

// -------------------------------------------------------------
// Driver
// -------------------------------------------------------------

fn continue_program() {
    println!("\n==============================");
    println!("Running Part 2");
    println!("==============================");

    print_result(safe_divide(20.0, 2.0));
    print_result(safe_divide(20.0, 0.0));

    demonstrate_result_methods();

    demonstrate_option();

    lifetime_demo();

    static_demo();

    module_demo();

    show_item(vec![1, 2, 3, 4, 5]);

    show_item(("Rust", 2026));

    println!("\nEnd of Part 2");
    println!("Continue with Part 3 below this code.");
}

/*
Add this line at the END of your existing main() from Part 1:

    continue_program();

This keeps everything in one file.
*/



// =============================================================
// main.rs (Part 3)
// Final Revision of Modules 6–10
// =============================================================

// -------------------------------------------------------------
// Trait Objects
// -------------------------------------------------------------

fn print_using_trait_object(item: &dyn Printable) {
    println!("\n=== Trait Object Demo ===");
    item.print();
}

// -------------------------------------------------------------
// Multiple Trait Bounds
// -------------------------------------------------------------

fn compare_and_print<T>(a: T, b: T)
where
    T: PartialOrd + Debug + Copy,
{
    println!("\n=== Trait Bounds Demo ===");

    if a > b {
        println!("{:?} is greater", a);
    } else {
        println!("{:?} is greater", b);
    }
}

// -------------------------------------------------------------
// Generic Repository
// -------------------------------------------------------------

#[derive(Debug)]
struct Repository<T> {
    data: Vec<T>,
}

impl<T> Repository<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn add(&mut self, item: T) {
        self.data.push(item);
    }

    fn count(&self) -> usize {
        self.data.len()
    }
}

// -------------------------------------------------------------
// Lifetime Method
// -------------------------------------------------------------

impl<'a> Book<'a> {
    fn get_title(&self) -> &'a str {
        self.title
    }
}

// -------------------------------------------------------------
// Option + Result Together
// -------------------------------------------------------------

fn get_first_number(numbers: &[i32]) -> Result<i32, String> {
    match numbers.first() {
        Some(value) => Ok(*value),
        None => Err(String::from("Vector is empty")),
    }
}

// -------------------------------------------------------------
// Final Revision
// -------------------------------------------------------------

fn final_revision() {

    println!("\n==============================");
    println!("Final Revision (Modules 6-10)");
    println!("==============================");

    //----------------------------------------------------------
    // Trait Object
    //----------------------------------------------------------

    let student = Student {
        id: 999,
        name: String::from("Revision Student"),
        cgpa: 9.9,
    };

    print_using_trait_object(&student);

    //----------------------------------------------------------
    // Trait Bounds
    //----------------------------------------------------------

    compare_and_print(100, 250);

    //----------------------------------------------------------
    // Generic Repository
    //----------------------------------------------------------

    let mut repository = Repository::new();

    repository.add(student);

    println!("Repository Count = {}", repository.count());

    //----------------------------------------------------------
    // Lifetime Method
    //----------------------------------------------------------

    let rust_book = Book {
        title: "Rust Programming",
        author: "Steve",
    };

    println!("Book Title = {}", rust_book.get_title());

    //----------------------------------------------------------
    // Result + Option
    //----------------------------------------------------------

    match get_first_number(&[10,20,30]) {

        Ok(value) => println!("First Number = {}", value),

        Err(error) => println!("{}", error),

    }

    //----------------------------------------------------------
    // Generic Struct Again
    //----------------------------------------------------------

    let container = Container::new(500);

    println!("Container Value = {}", container.get());

    //----------------------------------------------------------
    // Generic Pair
    //----------------------------------------------------------

    let pair = Pair {
        first: "Rust",
        second: 1.85,
    };

    pair.show();

    //----------------------------------------------------------
    // Summary
    //----------------------------------------------------------

    println!("\n✔ Traits");
    println!("✔ Generics");
    println!("✔ Trait Bounds");
    println!("✔ Generic Structs");
    println!("✔ Result");
    println!("✔ Option");
    println!("✔ Lifetimes");
    println!("✔ Trait Objects");
    println!("✔ Modules");
    println!("✔ Debug Trait");

    println!("\n🎉 Modules 6–10 Completed Successfully!");
}