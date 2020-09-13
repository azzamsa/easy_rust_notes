fn main() {
    let name = "azzam"; // it's an &str
    let name1 = String::from("azzam"); // it's a String
    let name2 = format!("is I am {} or {}", name, name1);

    println!("{}, {}, {}", name, name1, name2);
}

// NOTE:
//  &str is a simple string. A &str is very fast.
// String is a more complicated. It is a bit slower, but it has more functions. A String is a pointer, with data on the heap.
