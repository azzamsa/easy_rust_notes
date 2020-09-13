// fn: function
// main: function name
// (): empty arguments
fn main() {
    let name = "azzam";
    println!("Hello, {}!", name);

    let number = get_number();
    println!("number: {}", number);


    println!("name: {}", get_name(String::from("world")));
}

fn get_number() -> i32{
    // without ; it will be return value
   8
}

fn get_name(name: String) -> String{
    // without ; it will be return value
   name
}

// NOTE:
// variable lifetimes delimited by code-block
