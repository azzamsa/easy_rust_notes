fn main() {
    let mut counter = 0; // set a counter to 0
    loop {
        counter += 1; // increase the counter by 1
        println!("The counter is now: {}", counter);
        if counter == 5 {
            // stop when counter == 5
            break;
        }
    }

    println!("----");
    let mut counter1 = 0;

    while counter1 < 5 {
        counter1 += 1;
        println!("The counter is now: {}", counter1);
    }

    println!("----");
    for number in 0..3 {
        println!("The number is: {}", number);
    }
}

// NOTE:
// use `loop {}` for infinite-loop
// you can give name to loop
// `break foo` will break and return foo as value
