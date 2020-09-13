use std::fmt::Display;
use std::cmp::PartialOrd;

fn return_number<MyType>(number: MyType) -> MyType {
    println!("Here is your number.");
    number
}

// Display: make us able to use print with {}
// PartialOrd: make us able to use <, <, and so on.
fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
    println!("{}! Is {} greater than {}? {}", statement, num_1, num_2, num_1 > num_2);
}


fn main() {
    let number = return_number(5);
    println!("{}", number);

    compare_and_display("Listen up!", 9, 8);
}

// NOTE:
// use `where` if you have many generic types
