fn main() {
    let random_tuple = (
        "I am a &str",
        8,
        vec!['a'],
        String::from("String"),
        [8, 9, 10],
        7.7,
    );
    println!("{:?}", random_tuple);

    // access them using dot
    println!("first item: {}", random_tuple.0);
    println!("second item: {}", random_tuple.1);
}

// NOTE
// - tuple can hold different types
// - can be used to return multiple value or assigning multiple values
// use _ for unused variable while desctructuring
