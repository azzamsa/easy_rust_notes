fn main() {
    let mut my_number = 8;
    let num_ref = &mut my_number; // mutable reference: to use change value of referenced-variable
    *num_ref += 10; // *: to dereference
    println!("{}", my_number);

    // giving reference to function
    let country = String::from("Austria");
    print_country(&country); // We print "Austria"
    print_country(&country); // can't do twice because string passed to `print country`. Use &.

    let mut country1 = String::from("Austria");
    add_hungary(&mut country1);
}

fn print_country(country_name: &String) {
    println!("{}", country_name);
}

fn add_hungary(country_name: &mut String) {
    // mutable reference
    country_name.push_str("-Hungary"); //
    println!("Now it says: {}", country_name);
}

// NOTE:
// const is a value that does not change,
// static is a value that does not change and has a fixed memory location.
