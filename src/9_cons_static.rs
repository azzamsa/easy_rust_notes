fn main() {
    const NUMBER_OF_MONTHS: u32 = 12;
    static SEASONS: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];

    println!("{}", NUMBER_OF_MONTHS);
    println!("{:?}", SEASONS);
}

// NOTE:
// const is a value that does not change,
// static is a value that does not change and has a fixed memory location.
