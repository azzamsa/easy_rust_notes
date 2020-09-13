
fn main() {
    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);  // No problem, because my_number is copy type!. Can't do it with String.

    let country = String::from("Pamekasan");
    prints_country(country.clone()); // make a clone and give it to the function. Only the clone goes in, and country is still alive
    prints_country(country);
}

fn prints_number(number: i32) { // There is no -> so it's not returning anything
                             // If number was not copy type, it would take it
                             // and we couldn't use it again
    println!("{}", number);
}

fn prints_country(country_name: String) {
    println!("{}", country_name);
}

// NOTE
// .clone() is slow. use & if you can.
