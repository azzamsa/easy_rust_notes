// unwrap is also good when you are writing your program and you want it to
// crash when there is a problem. Later, when your code is finished it is good
// to change unwrap to something else that won't crash


fn get_fourth(input: &Vec<i32>) -> i32 {
    let fourth = input.get(3).expect("Input vector needs at least 4 items");
    *fourth
}

fn main() {
    let my_vec = vec![9, 0, 10];
    get_fourth(&my_vec);
}

// NOTE:
// expect() is better than unwrap()
// use `unwrap_or()` if you don't want your code never panic
// but it might be not good if you want the program to panic if there's a problem.
