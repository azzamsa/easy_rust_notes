fn main() {
    let mut name = "azzam"; // immutable by default
    println!("Hello, {}!", name);

    name = "world";
    println!("Hello, {}!", name);

    // shadow.
    println!("---");
    let name1 = "azzam"; // immutable by default
    println!("Hello, {}!", name1);

    {
        // only lives in this scope
        let name1 = "world";
        println!("Hello, {}!", name1);
    }

    println!("Hello, {}!", name1);
}
