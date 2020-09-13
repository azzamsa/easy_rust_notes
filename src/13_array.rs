fn main() {
    let my_numbers = [1, 2, 3];
    println!("my numbers: {:?}", my_numbers);

    let my_numbers1 = ["foo"; 5];
    println!("my numbers: {:?}", my_numbers1);

    // getting a slice
    // use &. because compiler doesn't know the size
    let my_numbers2 = [1, 2, 3, 4, 5];
    println!("my slice: {:?}", &my_numbers2[0..1]);
    println!("my slice: {:?}", &my_numbers2[0..3]);
    println!("my slice: {:?}", &my_numbers2[..]);
}

// NOTE
// - very fast
// - value can't be changed
// - must be the same type
