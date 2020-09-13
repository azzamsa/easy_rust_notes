// You use Option when you have a value that might exist, or might not exist.
// Option is about Some or None (value or no value),
fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 4 {
        None
    } else {
        Some(value[4])
    }
}

fn main() {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    let vec_of_vecs = vec![new_vec, bigger_vec];
    for vec in vec_of_vecs {
        let inside_number = take_fifth(vec);
        if inside_number.is_some() {
            // .is_some() returns true if we get Some, false if we get None
            println!("We got: {}", inside_number.unwrap()); // now it is safe to use .unwrap() because we already checked
        } else {
            println!("We got nothing.");
        }
    }
}

// NOTE:
// You can use .unwrap() to take the Option value.
// there is also .is_some and is_none
