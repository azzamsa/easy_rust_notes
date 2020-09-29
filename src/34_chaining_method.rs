// Even Rust is System Programming. It has a functional style.
// Chaining method make the code shorter.

// Such `.collect()`

fn main() {
    let new_vec = (1..=10).collect::<Vec<i32>>();
    println!("{:?}", new_vec);

    // Or you can write it like this:
    let new_vec1: Vec<i32> = (1..=10).collect(); // it's more readable for me
    println!("{:?}", new_vec1);

    let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let new_vec: Vec<i32> = my_vec
        .into_iter() // "iterate" over the items (iterate = work with each item
        // inside it). into_iter() gives us owned values, not references
        .skip(3) // skip over three items: 0, 1, and 2
        .take(4) // take the next four: 3, 4, 5, and 6
        .collect(); // put them in a new Vec<i32>

    println!("{:?}", new_vec);
}
