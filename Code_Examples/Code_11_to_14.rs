// =============================================================
// Module 11 - 14 Final Revision
// =============================================================

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

// -------------------------------------------------------------
// Module 11 : Closures
// -------------------------------------------------------------

fn closure_demo() {
    println!("\n========== Closures ==========");

    let add = |a: i32, b: i32| a + b;

    println!("10 + 20 = {}", add(10, 20));

    let tax = 18;

    // Closure captures tax automatically
    let total_price = |price: i32| price + tax;

    println!("Total Price = {}", total_price(100));
}

// -------------------------------------------------------------
// Module 11 : Iterators
// -------------------------------------------------------------

fn iterator_demo() {
    println!("\n========== Iterators ==========");

    let numbers = vec![1,2,3,4,5,6];

    let result: Vec<_> = numbers
        .iter()
        .filter(|x| **x % 2 == 0)
        .map(|x| x * 10)
        .collect();

    println!("Iterator Output = {:?}", result);

    let sum:i32 = numbers.iter().sum();

    println!("Sum = {}", sum);
}

// -------------------------------------------------------------
// Module 12 : Smart Pointers
// -------------------------------------------------------------

fn smart_pointer_demo() {

    println!("\n========== Smart Pointers ==========");

    //---------------- Box ----------------

    let value = Box::new(500);

    println!("Box Value = {}", value);

    //---------------- Rc ----------------

    let language = Rc::new(String::from("Rust"));

    let copy1 = Rc::clone(&language);
    let copy2 = Rc::clone(&language);

    println!("Rc Count = {}", Rc::strong_count(&language));

    println!("{}", copy1);
    println!("{}", copy2);

    //---------------- Arc ----------------

    let shared = Arc::new(vec![10,20,30]);

    println!("Arc Data = {:?}", shared);

    //---------------- RefCell ----------------

    let counter = RefCell::new(0);

    *counter.borrow_mut() += 1;

    println!("Counter = {}", counter.borrow());
}

// -------------------------------------------------------------
// Module 13 : Macros
// -------------------------------------------------------------

macro_rules! square {

    ($x:expr) => {
        $x * $x
    };

}

macro_rules! greet {

    () => {
        println!("Hello from Custom Macro!");
    };

}

fn macro_demo() {

    println!("\n========== Macros ==========");

    greet!();

    println!("Square = {}", square!(12));

    let numbers = vec![10,20,30];

    println!("{:?}", numbers);
}

// -------------------------------------------------------------
// Module 14 : Rust for Solana
// -------------------------------------------------------------

#[derive(Debug)]
struct UserAccount {

    owner:String,

    balance:u64,

}

// Solana instructions are often represented using enums.
#[derive(Debug)]
enum Instruction {

    Initialize,

    Deposit(u64),

    Withdraw(u64),

}

fn execute_instruction(account:&mut UserAccount, instruction:Instruction){

    match instruction{

        Instruction::Initialize=>{

            account.balance=0;

        }

        Instruction::Deposit(amount)=>{

            account.balance += amount;

        }

        Instruction::Withdraw(amount)=>{

            if account.balance>=amount{

                account.balance-=amount;

            }else{

                println!("Insufficient Funds");

            }

        }

    }

}

// Very small serialization example.
// (Real Solana uses Borsh or bincode.)
fn serialize(account:&UserAccount)->String{

    format!("{},{}",account.owner,account.balance)

}

fn deserialize(data:&str)->UserAccount{

    let parts:Vec<&str>=data.split(',').collect();

    UserAccount{

        owner:parts[0].to_string(),

        balance:parts[1].parse().unwrap(),

    }

}

fn solana_demo(){

    println!("\n========== Rust for Solana ==========");

    let mut account=UserAccount{

        owner:String::from("Shivansh"),

        balance:1000,

    };

    execute_instruction(

        &mut account,

        Instruction::Deposit(500),

    );

    execute_instruction(

        &mut account,

        Instruction::Withdraw(300),

    );

    println!("{:?}",account);

    let bytes=serialize(&account);

    println!("Serialized = {}",bytes);

    let restored=deserialize(&bytes);

    println!("Deserialized = {:?}",restored);
}

// -------------------------------------------------------------
// Driver
// -------------------------------------------------------------

fn module11_to_14_demo(){

    println!("\n==================================");
    println!("Modules 11 - 14");
    println!("==================================");

    closure_demo();

    iterator_demo();

    smart_pointer_demo();

    macro_demo();

    solana_demo();

    println!("\nModules 11-14 Completed ");
}