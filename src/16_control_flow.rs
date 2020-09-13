fn main() {
    let my_number = 5;
    if my_number == 7 {
        println!("It's seven");
    } else if my_number == 6 {
        println!("It's six")
    } else {
        println!("It's a different number")
    }

    let my_number1: u8 = 5;
    match my_number1 {
        0 => println!("it's zero"), // each line is called an 'arm'
        1 => println!("it's one"),
        2 => println!("it's two"),
        _ => println!("It's some other number"),
    }

    let sky = "cloudy";
    let temperature = "warm";
    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's dark and unpleasant today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", "warm") => println!("It's dark but not bad"),
        _ => println!("Not sure what the weather is."),
    }
}

// NOTE
// You an also use || and &&
// You can use if inside match
// You can use _ as many times in match
// A match has to return the same type.
