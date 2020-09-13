/* Result is about Ok or Err (okay result, or error result).

So Option is if you are thinking: "Maybe there will be something, and maybe
there won't." But Result is if you are thinking: "Maybe it will fail."

 */

fn give_result(input: i32) -> Result<(), ()> {
    // returning empty tuples
    if input % 2 == 0 {
        return Ok(());
    } else {
        return Err(());
    }
}

fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err("Sorry, the number wasn't five.".to_string()), // This is our error message
    }
}

fn main() {
    if give_result(5).is_ok() {
        println!("It's okay, guys")
    } else {
        println!("It's an error, guys")
    }

    println!("---");

    let mut result_vec = Vec::new(); // Create a new vec for the results
    for number in 3..6 {
        result_vec.push(check_if_five(number)); // push each result into the vec
    }

    println!("{:?}", result_vec);
}
