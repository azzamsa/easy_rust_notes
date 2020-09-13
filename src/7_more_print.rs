fn main() {
    print!("this is \t tab");

    println!(
        "this
is
multiple
lines"
    );

    println!("Here are two escape characters: \\n and \\t");

    // you can use `r#` as many as you want to escape strings
    // with `r#` you can also escape reserved keywords
    println!(r#"Here are two escape characters: \n and \t"#);

    // `b` convert as bytes
    println!("{:?}", b"This will look like numbers");

    let number = 1;
    let number_ref = &number;
    println!("{:p}", number_ref); // pointer address

    let number1 = 555;
    println!(
        "Binary: {:b}, hexadecimal: {:x}, octal: {:o}",
        number1, number1, number1
    );

    println!(
        "{city1} is not in {country}.",
        city1 = "Seoul",
        country = "Korea"
    );

    println!("{:-^30}", "HALLO");
    println!("{: <15}{: >15}", "|", "|");
}

// NOTE:
// {} (for Display) and {:?} (for Debug), plus {:#?} for pretty printing.
