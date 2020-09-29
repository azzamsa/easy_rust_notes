// You can use AsRef to limit Generic signature to only limit &str and String

use std::fmt::Display;

fn print_it<T: AsRef<str>  + Display>(input: T) {
    println!("{}", input)
}

fn main() {
    print_it("Please print me");
    print_it("Please print me".to_string());
}

// NOTE:
// use `where` if your signature get longer
