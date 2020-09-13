// if let means "do something if it matches, and don't do anything if it doesn't"
// if let is when you don't care about matching for everything.

fn main() {
    let my_vec = vec![2, 3, 4];

    for index in 0..10 {
        match my_vec.get(index) {
            Some(number) => println!("The number is: {}", number),
            None => {}
        }
    }

    println!("---");
    // instead. we can do
    for index in 0..10 {
        if let Some(number) = my_vec.get(index) {
            println!("The number is: {}", number);
        }
    }
}
