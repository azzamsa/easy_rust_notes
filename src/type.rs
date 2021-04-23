#[allow(unused_variables)]

pub fn main() {
    // Primitive types (basic): int, char,
    let one = 1;

    // Type inference
    let two = 2; // rust will decide the type if it can. so writing type is optional

    // for integer and float, there is *singed* and *un-singed* type

    // casting type
    let two_f32 = two as f32;
    // you can add `_` to long number to make it more readable
    let long_num = 1_123_000;

    println!("{}", one);
    println!("{}", two_f32);
}
