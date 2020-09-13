fn main() {
    let mut my_belonging: Vec<String> = Vec::new();
    my_belonging.push(String::from("celana"));
    my_belonging.push(String::from("baju"));

    println!("my belonging: {:?}", my_belonging);

    let pronouns = vec!["aku", "kamu", "dia"];
    println!("pronouns: {:?}", pronouns);
    println!("pronouns slice: {:?}", &pronouns[0..2]);

    // you can check it's capacity using .capacity()
    // it will reallocate itself. from 4-8-so on
    // you can also specify the capacity beforehand
    let mut num_vec = Vec::with_capacity(8);
    num_vec.push('a'); // add one character
    println!("{}", num_vec.capacity()); // prints 8


}

// NOTE
// you can use .into() to make &str to String, and array into vec
