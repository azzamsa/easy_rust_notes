mod comment;
mod r#type;
mod function;
mod display_debug;

fn main() {
    println!("1. Comment");
    comment::main();

    println!("\n2. Fype");
    r#type::main();

    println!("\n3. Function");
    function::main();

    println!("\n4. Display & Debug");
    display_debug::main();

}
