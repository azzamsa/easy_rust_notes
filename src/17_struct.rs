// With structs, you can create your own type.

struct FileDirectory; // unit
struct Colour(u8, u8, u8); // tuple or unnamed

struct SizeAndColour {
    // named struct
    size: u32,
    colour: Colour,
}

fn main() {
    let my_colour = Colour(50, 0, 50); // Make a colour out of RGB (red, green, blue)
    println!("The second part of the colour is: {}", my_colour.1);

    let size_and_colour = SizeAndColour {
        size: 150,
        colour: my_colour,
    };
    println!("size : {}", size_and_colour.size); // access the value with dot
}

