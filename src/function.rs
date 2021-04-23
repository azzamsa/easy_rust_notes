// fn: function
// main: function name
// (): empty arguments
pub fn main() {
    let name = "ferris";
    println!("Hello, {}!", name);

    let number = get_number();
    println!("number: {}", number);

    println!("name: {}", get_name(String::from("world")));
}

fn get_number() -> i32{
    // without ; it will be return value
   8
}

// function with one parameter
fn get_name(name: String) -> String{
    // you can also use `return`
   return name;
}

// NOTE:
// variable lifetimes delimited by code-block
